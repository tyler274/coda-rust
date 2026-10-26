# synthetic_hospital patient 1979

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, African American, Medicare (ESRD-qualified) and Medicaid dual-eligibility.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-16 (Mon) | outpatient | Endocrinology |
| 1 | 2020-05-15 | 2022-09-16 (Fri) | outpatient | Nephrology |
| 2 | 2020-09-15 | 2023-01-12 (Thu) | outpatient | Nephrology |
| 3 | 2021-03-15 | 2023-07-12 (Wed) | outpatient | Internal Medicine |
| 4 | 2021-09-15 | 2024-01-26 (Fri) | outpatient | Ophthalmology |
| 5 | 2022-05-15 | 2024-09-11 (Wed) | outpatient | Orthopedics |
| 6 | 2023-03-15 | 2025-07-14 (Mon) | ed | Emergency Medicine |
| 7 | 2023-11-15 | 2026-03-12 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (ESRD-qualified) and Medicaid dual-eligibility",
 "occupation": "Delivery driver (local package delivery, requires prolonged standing/walking)",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus with diabetic nephropathy",
  "Mother: Hypertension and stroke (age 68)",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet PO daily (nonadherent, misses 2-3 doses per week)",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy)"
  },
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale subcutaneous with meals (frequently missed)",
   "name": "Insulin lispro"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "800 mg PO three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "0.25 mcg PO daily",
   "name": "Calcitriol"
  },
  {
   "dose": "4000 units IV three times weekly with dialysis sessions",
   "name": "Epoetin alfa"
  },
  {
   "dose": "20 mg IM monthly (recently initiated for acromegaly)",
   "name": "Octreotide LAR"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Tunneled hemodialysis catheter placement (14 months ago)",
  "Right ankle arthrodesis (pending)",
  "Pan-retinal photocoagulation laser therapy (history of)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (20-year duration, poorly controlled)",
  "Hypertension",
  "HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)",
  "End-Stage Renal Disease on hemodialysis",
  "Chronic Kidney Disease Stage 5",
  "Diabetic retinopathy",
  "Peripheral neuropathy (severe, sensorimotor)",
  "Heart failure with reduced ejection fraction",
  "Secondary hyperparathyroidism of renal origin",
  "Acromegaly",
  "Charcot neuroarthropathy (right ankle/foot)"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["HIV-related opportunistic skin infection", "Diabetic retinopathy", "Heart failure with reduced ejection fraction", "Diabetic nephropathy", "Catheter-related bloodstream infection", "Secondary hyperparathyroidism of renal origin", "Neurogenic arthropathy (Charcot joint)", "Acromegaly"]
Comorbidities: ["Medication noncompliance", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-05-16, outpatient, Endocrinology

Chief complaint: Persistent fatigue and joint stiffness with difficulty gripping objects

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and joint stiffness with difficulty gripping objects

HISTORY OF PRESENT ILLNESS:
Persistent fatigue and joint stiffness that has been worsening over the past year. He reports difficulty gripping objects and says his hands feel "tight" and "clumsy." He attributes it to aging but has recently noticed that he frequently drops his phone and that his wrists hurt more in the evenings.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Diabetic retinopathy
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Secondary hyperparathyroidism of renal origin
- Acromegaly
- Charcot neuroarthropathy (right ankle/foot)

The patient has a history of newly diagnosed hypertension and elevated fasting glucose but has not yet started treatment.

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Delivery driver (local package delivery, requires prolonged standing/walking).

REVIEW OF SYSTEMS:
He denies any fever, rash, or recent illness. He mentions occasional palpitations, but no chest pain or dyspnea.

PHYSICAL EXAMINATION:
Physical examination shows mildly elevated blood pressure, thickened skin over the fingers, and subtle widening of the hands. There is no visible swelling or erythema, and heart and lung exams are normal. Neurologic exam reveals decreased sensation over the first three digits bilaterally. His facial exam is notable for deepening of the nasolabial folds, but no obvious asymmetry or lesions.

LABORATORY DATA:
Laboratory tests are pending.

## Encounter 1: source 2020-05-15, practice 2022-09-16, outpatient, Nephrology

Chief complaint: Progressive swelling in both legs and foamy urine

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling in both legs and foamy urine

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with Type 2 Diabetes Mellitus, End-Stage Renal Disease on hemodialysis, HIV infection, and acromegaly diagnosed during recent Endocrinology evaluation presents for follow-up assessment. He reports poor adherence to medications and dietary restrictions.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Diabetic retinopathy
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Secondary hyperparathyroidism of renal origin
- Charcot neuroarthropathy (right ankle/foot)

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Delivery driver (local package delivery, requires prolonged standing/walking).

VITAL SIGNS:
His blood pressure is 148/94 mmHg.

PHYSICAL EXAMINATION:
On examination, he has bilateral pitting edema to the knees.

LABORATORY DATA:
Urinalysis reveals 4+ proteinuria, and serum albumin is decreased.

PATHOLOGY:
A kidney biopsy is performed and a representative histologic image is shown below. The physician recognizes that this biopsy demonstrates an advanced feature of diabetic nephropathy and recalls that a characteristic renal hemodynamic change occurred early in this disease.

## Encounter 2: source 2020-09-15, practice 2023-01-12, outpatient, Nephrology

Chief complaint: Routine follow-up for ESRD with mild fatigue

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for ESRD with mild fatigue

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with end-stage renal disease on maintenance hemodialysis and recently diagnosed acromegaly returns for routine nephrology follow-up subsequent to prior evaluation for diabetic nephropathy. He reports mild fatigue but no significant new symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Diabetic nephropathy (diagnosed 2020-05-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Diabetic retinopathy
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Secondary hyperparathyroidism of renal origin
- Charcot neuroarthropathy (right ankle/foot)

He has a history of type 2 diabetes mellitus and chronic hypertension.

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
His medications include lisinopril and metformin.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Delivery driver (local package delivery, requires prolonged standing/walking).

PHYSICAL EXAMINATION:
On exam, he appears well and physical findings are unremarkable.

LABORATORY DATA:
Laboratory studies, including serum creatinine, are pending. The following nomogram represents his serum parathyroid hormone (PTH) and calcium concentrations.

## Encounter 3: source 2021-03-15, practice 2023-07-12, outpatient, Internal Medicine

Chief complaint: Increasingly forgetful with difficulty concentrating and social withdrawal

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increasingly forgetful with difficulty concentrating and social withdrawal

HISTORY OF PRESENT ILLNESS:
Six months following Nephrology evaluation for secondary hyperparathyroidism, the patient, a 58-year-old with acromegaly, end-stage renal disease on hemodialysis, and HIV infection, presents to clinic with sister, who reports that over the past six months, he has become increasingly forgetful and has difficulty concentrating. She also notes that he has been less interested in his hobbies and has become socially withdrawn.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Diabetic nephropathy (diagnosed 2020-05-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-09-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Diabetic retinopathy
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Charcot neuroarthropathy (right ankle/foot)

His medical history is significant for hypertension and a recent diagnosis of type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
He has been on antiretroviral therapy (ART) for the past 10 years but admits to occasional nonadherence.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Delivery driver (local package delivery, requires prolonged standing/walking).

VITAL SIGNS:
Blood pressure: 140/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient appears apathetic and has a flat affect. He is oriented to person and place but not to time. Neurological examination reveals slowed psychomotor speed and impaired recall.

LABORATORY DATA:
CD4 count: 180/mm3
HIV viral load: 55,000 copies/mL
Fasting glucose: 160 mg/dL
Hemoglobin A1c: 7.8%

## Encounter 4: source 2021-09-15, practice 2024-01-26, outpatient, Ophthalmology

Chief complaint: Progressive visual difficulty and fluctuating blurry vision

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James Thompson, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive visual difficulty and fluctuating blurry vision

HISTORY OF PRESENT ILLNESS:
The patient with 20-year poorly controlled Type 2 Diabetes Mellitus returns for ophthalmologic follow-up six months after Internal Medicine. He reports that his vision sometimes becomes blurry or fluctuates during the day, and he often struggles to recognize facial features and distinguish colors, especially in dim lighting. He denies any eye pain, photophobia, or flashes of light.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Diabetic nephropathy (diagnosed 2020-05-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-09-15)
- HIV-related opportunistic skin infection (diagnosed 2021-03-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Diabetic retinopathy
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Charcot neuroarthropathy (right ankle/foot)

He has a history of type 2 diabetes mellitus for over 10 years.

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
He admits to inconsistently taking his medications.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
He does not smoke but has been sedentary and has missed several ophthalmology appointments over the last two years.

VITAL SIGNS:
Blood pressure today is 142/88 mm Hg.

PHYSICAL EXAMINATION:
Visual acuity testing reveals moderate bilateral reduction in sharpness. Fundoscopic examination is performed, and the image below is obtained.

## Encounter 5: source 2022-05-15, practice 2024-09-11, outpatient, Orthopedics

Chief complaint: Progressive swelling and deformity of right foot over past 8 months with mechanical grinding

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Maria Gonzalez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling and deformity of right foot over past 8 months with mechanical grinding

HISTORY OF PRESENT ILLNESS:
The patient presents for evaluation of progressive right foot deformity in the context of longstanding Type 2 Diabetes Mellitus with severe peripheral neuropathy, end-stage renal disease on hemodialysis, and acromegaly followed in Endocrinology since 2020. He states that the foot has been “changing shape” and that he sometimes hears cracking or grinding when he walks, but feels little to no discomfort. He reports that approximately nine months ago, he twisted his ankle while walking on uneven pavement but continued his daily routine because the pain was minimal. Since that time, the swelling has gradually worsened, and the foot now looks “crooked” to him. His wife reports that he sometimes walks with an unusual gait but does not complain of pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Diabetic nephropathy (diagnosed 2020-05-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-09-15)
- HIV-related opportunistic skin infection (diagnosed 2021-03-15)
- Diabetic retinopathy (diagnosed 2021-09-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Charcot neuroarthropathy (right ankle/foot)

He has a history of poorly controlled type 2 diabetes mellitus for 20 years and peripheral neuropathy diagnosed 5 years ago. He also has diabetic retinopathy, chronic kidney disease stage 3, and hypertension.

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
The patient works as a delivery driver and spends many hours standing or walking each day. He often wears old, worn-out work boots.

REVIEW OF SYSTEMS:
He has not had any fevers, chills, or recent infections. Review of systems is otherwise unremarkable.

PHYSICAL EXAMINATION:
On physical examination, the right foot appears markedly swollen, warm, and erythematous with obvious midfoot collapse and deformity. There is joint instability, bony crepitus on palpation, and reduced active range of motion. Achilles tendon reflexes are absent bilaterally. Dorsalis pedis and posterior tibial pulses are 2+ bilaterally. The left foot appears normal.

LABORATORY DATA:
Laboratory studies, including complete blood count, erythrocyte sedimentation rate, and C-reactive protein, are within normal limits.

IMAGING:
Plain radiographs of the right foot demonstrate bone fragmentation, joint subluxation, and sclerosis consistent with advanced joint destruction.

OTHER STUDIES:
There is no joint aspiration performed as there are no signs of infection.

## Encounter 6: source 2023-03-15, practice 2025-07-14, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, orthopnea, and lower extremity edema

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, orthopnea, and lower extremity edema

HISTORY OF PRESENT ILLNESS:
This 60-year-old with heart failure with reduced ejection fraction and end-stage renal disease on hemodialysis, with Nephrology care, presents to the emergency department with 2-day history of increasing shortness of breath, orthopnea, and lower extremity edema. He reports recent increase in his salt intake and missing several doses of his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Diabetic nephropathy (diagnosed 2020-05-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-09-15)
- HIV-related opportunistic skin infection (diagnosed 2021-03-15)
- Diabetic retinopathy (diagnosed 2021-09-15)
- Neurogenic arthropathy (Charcot joint) (diagnosed 2022-05-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Peripheral neuropathy (severe, sensorimotor)
- Heart failure with reduced ejection fraction
- Charcot neuroarthropathy (right ankle/foot)

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Delivery driver (local package delivery, requires prolonged standing/walking).

VITAL SIGNS:
On physical examination, his blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 90% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral rales halfway up both lung fields. Cardiac examination is notable for an S3 gallop. Jugular venous pressure is elevated. Extremities show 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Laboratory results are as follows:

- Serum sodium: 138 mEq/L
- Serum potassium: 4.9 mEq/L
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL

IMAGING:
Chest X-ray demonstrates cardiomegaly and pulmonary congestion.

## Encounter 7: source 2023-11-15, practice 2026-03-12, ed, Emergency Medicine

Chief complaint: Fever and chills with temperature 101.5°F

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever and chills with temperature 101.5°F

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with Type 2 Diabetes, HIV, ESRD on hemodialysis, Charcot arthropathy, and heart failure, presents to the emergency department with a 1-day history of fever and chills. He reports a temperature of 101.5°F at home. He has a tunneled hemodialysis catheter in place for the past 14 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Acromegaly (diagnosed 2020-01-15)
- Diabetic nephropathy (diagnosed 2020-05-15)
- Secondary hyperparathyroidism of renal origin (diagnosed 2020-09-15)
- HIV-related opportunistic skin infection (diagnosed 2021-03-15)
- Diabetic retinopathy (diagnosed 2021-09-15)
- Neurogenic arthropathy (Charcot joint) (diagnosed 2022-05-15)
- Heart failure with reduced ejection fraction (diagnosed 2023-03-15)
- Type 2 Diabetes Mellitus (20-year duration, poorly controlled)
- Hypertension
- HIV infection (diagnosed 10 years ago, on intermittent antiretroviral therapy)
- End-Stage Renal Disease on hemodialysis
- Chronic Kidney Disease Stage 5
- Peripheral neuropathy (severe, sensorimotor)
- Charcot neuroarthropathy (right ankle/foot)

His past medical history is significant for diabetes mellitus type 2 and hypertension.

PAST SURGICAL HISTORY:
- Tunneled hemodialysis catheter placement (14 months ago)
- Right ankle arthrodesis (pending)
- Pan-retinal photocoagulation laser therapy (history of)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 1 tablet PO daily (nonadherent, misses 2-3 doses per week)
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro Sliding scale subcutaneous with meals (frequently missed)
- Lisinopril 10 mg PO daily
- Carvedilol 12.5 mg PO twice daily
- Atorvastatin 40 mg PO daily
- Sevelamer carbonate 800 mg PO three times daily with meals
- Calcitriol 0.25 mcg PO daily
- Epoetin alfa 4000 units IV three times weekly with dialysis sessions
- Octreotide LAR 20 mg IM monthly (recently initiated for acromegaly)
- Aspirin 81 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension and stroke (age 68)
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Delivery driver (local package delivery, requires prolonged standing/walking).

REVIEW OF SYSTEMS:
He denies any cough, dysuria, diarrhea, or other localizing symptoms.

VITAL SIGNS:
On examination, his temperature is 102.2°F, blood pressure is 150/90 mmHg, heart rate is 102 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
The catheter exit site appears clean without erythema, swelling, or drainage.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 220,000/mm³ (Normal: 150,000-450,000/mm³)
- Blood cultures: Pending
- Urinalysis: Normal
