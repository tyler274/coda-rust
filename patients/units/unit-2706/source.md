# synthetic_hospital patient 2706

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex F, Non-Hispanic White, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-03-15 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2021-05-25 (Tue) | outpatient | Endocrinology |
| 2 | 2020-07-15 | 2021-09-20 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-01-15 | 2022-03-25 (Fri) | outpatient | Endocrinology |
| 4 | 2022-01-15 | 2023-04-03 (Mon) | outpatient | Dermatology |
| 5 | 2023-01-15 | 2024-03-19 (Tue) | outpatient | Otolaryngology |
| 6 | 2024-01-15 | 2025-03-19 (Wed) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-03-28 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "Accountant",
 "alcohol_use": "Occasional (1 glass of wine on weekends)",
 "family_history": [
  "Father: Hypertension, hyperlipidemia, coronary artery disease",
  "Mother: Breast cancer (age 62), autoimmune thyroiditis",
  "Paternal aunt: Systemic lupus erythematosus",
  "Older brother: Cutaneous malignant melanoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "200 mg PO daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.5 mg PO twice weekly (patient non-compliant)",
   "name": "Cabergoline"
  }
 ],
 "surgical_history": [
  "Wide local excision with sentinel lymph node biopsy of right lower leg melanoma",
  "Total thyroidectomy",
  "Renal biopsy"
 ],
 "chronic_conditions": [
  "Hypertension secondary to primary hyperaldosteronism",
  "Systemic lupus erythematosus",
  "Nephrotic syndrome",
  "Prolactinoma",
  "Mixed hyperlipidemia",
  "Lichen planus (vulvar)",
  "History of cutaneous malignant melanoma",
  "Medullary thyroid carcinoma"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Vulvar lichen planus", "Mixed hyperlipidemia", "Primary hyperaldosteronism", "Toxic single thyroid nodule", "Pituitary apoplexy with secondary adrenal insufficiency (adrenal crisis)", "Cutaneous malignant melanoma", "Renal vein thrombosis", "Malignant neoplasm of thyroid gland"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-03-15, outpatient, Internal Medicine

Chief complaint: Routine health check and cardiovascular risk assessment

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check and cardiovascular risk assessment

HISTORY OF PRESENT ILLNESS:
She is concerned about her cardiovascular risk and asks what she can do to help improve her numbers.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- Mixed hyperlipidemia
- Lichen planus (vulvar)
- History of cutaneous malignant melanoma
- Medullary thyroid carcinoma

She has a history of hypertension.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
She takes she takes amlodipine.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
She does not smoke and drinks one glass of wine on weekends. Her diet is mostly Mediterranean,

REVIEW OF SYSTEMS:
She reports no recent weight changes.

VITAL SIGNS:
Her BMI is 23 kg/m². Vitals are within normal limits.

LABORATORY DATA:
Fasting lipid panel shows:
    Total cholesterol                        245 mg/dL
    LDL cholesterol                          160 mg/dL
    HDL cholesterol                          37 mg/dL
    Triglycerides                            220 mg/dL

## Encounter 1: source 2020-03-15, practice 2021-05-25, outpatient, Endocrinology

Chief complaint: Progressive fatigue, generalized muscle weakness, and intermittent muscle cramps

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James O'Brien, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, generalized muscle weakness, and intermittent muscle cramps

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old female with hypertension secondary to primary hyperaldosteronism who returns for follow-up, having been seen in Internal Medicine two months prior for cardiovascular risk assessment. She reports difficulty carrying grocery bags and climbing stairs due to leg weakness. She works as an accountant and has recently noticed feeling lightheaded when standing up quickly, though denies palpitations, chest pain, or syncope. She mentions that her blood pressure readings at home are "always on the high side" despite reducing salt intake.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- Lichen planus (vulvar)
- History of cutaneous malignant melanoma
- Medullary thyroid carcinoma

Her medical history includes borderline hypertension diagnosed three years ago, for which she has not been taking any medications.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is unremarkable

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use recreational drugs.

VITAL SIGNS:
Temperature is 36.6°C (97.9°F), blood pressure is 166/94 mmHg, pulse is 72/min, and BMI is 24 kg/m²

PHYSICAL EXAMINATION:
Neurological examination reveals decreased deep tendon reflexes in the lower extremities. Cardiopulmonary and abdominal examinations are normal.

OTHER STUDIES:
ECG demonstrates flattened T waves and prominent U waves

## Encounter 2: source 2020-07-15, practice 2021-09-20, outpatient, Obstetrics & Gynecology

Chief complaint: Six-month history of persistent vulvar itching and discomfort

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of persistent vulvar itching and discomfort

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with systemic lupus erythematosus and history of vulvar lichen planus, seen recently in Endocrinology for hyperaldosteronism, presents to clinic with 6-month history of vulvar itching and discomfort. She reports symptoms are persistent and have progressively worsened. She has tried over-the-counter antifungal creams without relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-03-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- Lichen planus (vulvar)
- History of cutaneous malignant melanoma
- Medullary thyroid carcinoma

Her past medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
She denies any new sexual partners.

PHYSICAL EXAMINATION:
On physical examination, you note multiple glazed, erythematous erosions on the vulva bordered by white, lacy striae. Vaginal examination reveals similar lesions on the vaginal mucosa. An oral examination shows white, reticulated patches on the buccal mucosa.

## Encounter 3: source 2021-01-15, practice 2022-03-25, outpatient, Endocrinology

Chief complaint: Two-month history of palpable neck mass

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of palpable neck mass

HISTORY OF PRESENT ILLNESS:
Returning to Endocrinology with history of medullary thyroid carcinoma, she presents with a two-month neck mass. She reports no pain, dysphagia, or changes in her voice.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-03-15)
- Vulvar lichen planus (diagnosed 2020-07-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- History of cutaneous malignant melanoma
- Medullary thyroid carcinoma

Her past medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
She does not take any other medications

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 glass of wine on weekends). Occupation: Accountant.

PHYSICAL EXAMINATION:
On physical examination, a solitary, non-tender, 2 cm nodule is palpable in the right lobe of the thyroid. There are no signs of cervical lymphadenopathy. The rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum TSH: <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 2.5 ng/dL (normal range: 0.9-1.7 ng/dL)

IMAGING:
A thyroid ultrasound shows a solitary, hyperechoic, 2 cm nodule in the right thyroid lobe with increased intranodular blood flow.

## Encounter 4: source 2022-01-15, practice 2023-04-03, outpatient, Dermatology

Chief complaint: Evaluation of changing pigmented skin lesion on right lower leg

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of changing pigmented skin lesion on right lower leg

HISTORY OF PRESENT ILLNESS:
The patient returns for dermatology follow-up with history of melanoma and vulvar lichen planus. She reports that the lesion has been present for several years but has recently increased in size and changed in color. She denies any pain but mentions occasional itching.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-03-15)
- Vulvar lichen planus (diagnosed 2020-07-15)
- Toxic single thyroid nodule (diagnosed 2021-01-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- History of cutaneous malignant melanoma
- Medullary thyroid carcinoma

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 glass of wine on weekends). Occupation: Accountant.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
On examination, the lesion is 8 mm in diameter, with an asymmetrical shape, irregular borders, and multiple colors including brown, black, and a hint of blue. It is not tender to palpation. There are no similar lesions noted elsewhere on her body. The rest of the skin examination is unremarkable. and the remainder of the physical examination is within normal limits. No lymphadenopathy is palpable.

## Encounter 5: source 2023-01-15, practice 2024-03-19, outpatient, Otolaryngology

Chief complaint: Three-month history of palpable neck mass with recent onset of diarrhea

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of palpable neck mass with recent onset of diarrhea

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of a neck mass in the context of known medullary thyroid carcinoma and prior toxic thyroid nodule assessment in 2021. She denies any pain, dysphagia, or changes in her voice.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-03-15)
- Vulvar lichen planus (diagnosed 2020-07-15)
- Toxic single thyroid nodule (diagnosed 2021-01-15)
- Cutaneous malignant melanoma (diagnosed 2022-01-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- Medullary thyroid carcinoma

Her medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 glass of wine on weekends). Occupation: Accountant.

VITAL SIGNS:
Her blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
On physical examination, a firm, non-tender nodule is palpated in the central aspect of her neck. No cervical lymphadenopathy is appreciated. The rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Serum calcium: 9.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Thyroid-stimulating hormone (TSH): 2.1 µIU/mL (normal: 0.4-4.0 µIU/mL)
- Serum calcitonin: 450 pg/mL (normal: <10 pg/mL)
- Carcinoembryonic antigen (CEA): Elevated

IMAGING:
An ultrasound of the neck shows a 2 cm nodule in the thyroid gland with microcalcifications and irregular margins.

PATHOLOGY:
Fine-needle aspiration cytology is consistent with medullary thyroid cancer.

## Encounter 6: source 2024-01-15, practice 2025-03-19, ed, Emergency Medicine

Chief complaint: Sudden onset left flank pain and gross hematuria

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset left flank pain and gross hematuria

HISTORY OF PRESENT ILLNESS:
The patient with longstanding SLE and nephrotic syndrome presents acutely. She mentions that she has been feeling more fatigued over the past few weeks and has noticed some swelling in her legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-03-15)
- Vulvar lichen planus (diagnosed 2020-07-15)
- Toxic single thyroid nodule (diagnosed 2021-01-15)
- Cutaneous malignant melanoma (diagnosed 2022-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2023-01-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- Medullary thyroid carcinoma

She has a history of systemic lupus erythematosus and was diagnosed with nephrotic syndrome 2 years ago.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 glass of wine on weekends). Occupation: Accountant.

VITAL SIGNS:
On physical examination, her blood pressure is 145/90 mmHg, pulse is 88 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
Her left flank is tender to palpation. There is no costovertebral angle tenderness.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum creatinine: 1.8 mg/dL (baseline 1.0 mg/dL)
- Urine protein: 4+
- Urine RBC: 50-100/high power field
- Serum albumin: 2.5 g/dL
- Platelet count: 250,000/µL
- Prothrombin time: 12 seconds (control 11 seconds)
- Partial thromboplastin time: 30 seconds (control 28 seconds)

IMAGING:
A Doppler ultrasound of the renal vessels shows reduced flow in the left renal vein.

## Encounter 7: source 2025-01-15, practice 2026-03-28, ed, Emergency Medicine

Chief complaint: Sudden onset severe thunderclap headache with nausea and visual changes

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe thunderclap headache with nausea and visual changes

HISTORY OF PRESENT ILLNESS:
This 51-year-old female with prolactinoma and cabergoline noncompliance returns after renal vein thrombosis. She reports that the headache is the worst she has ever experienced and it started abruptly 2 hours ago. She also complains of bitemporal hemianopsia.

PAST MEDICAL HISTORY:
Active Problem List:
- Mixed hyperlipidemia (diagnosed 2020-01-15)
- Primary hyperaldosteronism (diagnosed 2020-03-15)
- Vulvar lichen planus (diagnosed 2020-07-15)
- Toxic single thyroid nodule (diagnosed 2021-01-15)
- Cutaneous malignant melanoma (diagnosed 2022-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2023-01-15)
- Renal vein thrombosis (diagnosed 2024-01-15)
- Hypertension secondary to primary hyperaldosteronism
- Systemic lupus erythematosus
- Nephrotic syndrome
- Prolactinoma
- Medullary thyroid carcinoma

Her past medical history is significant for hypertension and a prolactinoma for which she has been non-compliant with her dopamine agonist therapy.

PAST SURGICAL HISTORY:
- Wide local excision with sentinel lymph node biopsy of right lower leg melanoma
- Total thyroidectomy
- Renal biopsy

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Hydroxychloroquine 200 mg PO daily
- Atorvastatin 20 mg PO daily
- Cabergoline 0.5 mg PO twice weekly (patient non-compliant)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia, coronary artery disease
- Mother: Breast cancer (age 62), autoimmune thyroiditis
- Paternal aunt: Systemic lupus erythematosus
- Older brother: Cutaneous malignant melanoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 glass of wine on weekends). Occupation: Accountant.

VITAL SIGNS:
On examination, her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, and she is afebrile.

PHYSICAL EXAMINATION:
She appears lethargic but is oriented to person, place, and time. Cranial nerve examination reveals bitemporal hemianopsia without other deficits.

LABORATORY DATA:
Laboratory tests reveal the following:

- Sodium: 129 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.4 mEq/L (Normal: 3.5-5.1 mEq/L)
- Cortisol (8 AM): 2 µg/dL (Normal: 7-25 µg/dL)
- ACTH: Undetectable

ASSESSMENT AND PLAN:
Based on the clinical presentation and laboratory findings,
