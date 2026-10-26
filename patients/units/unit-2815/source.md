# synthetic_hospital patient 2815

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-06 (Mon) | outpatient | General Surgery |
| 1 | 2020-07-15 | 2026-03-30 (Mon) | outpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school science teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Fibrocystic breast disease",
  "Maternal aunt: Benign breast biopsy at age 45"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200-400 mg PO every 6-8 hours PRN for breast pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Right breast excisional biopsy (benign fibroadenoma) - 2019",
  "Left breast core needle biopsy (benign) - 2021"
 ],
 "chronic_conditions": [
  "Fibrocystic breast changes"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Simple breast cyst"]
Comorbidities: ["Complex breast cyst requiring surveillance"]

## Encounter 0: source 2020-01-15, practice 2025-10-06, outpatient, General Surgery

Chief complaint: Presents to the clinic with a palpable mass in her right breast that she discovered one month ago

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic with a palpable mass in her right breast that she discovered one month ago

HISTORY OF PRESENT ILLNESS:
The mass is non-tender and has not changed in size. She denies any nipple discharge, skin changes, or pain. Her last mammogram was 18 months ago and was normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Fibrocystic breast changes

Her medical history is significant for fibrocystic breast changes and two benign breast biopsies in the past.

PAST SURGICAL HISTORY:
- Right breast excisional biopsy (benign fibroadenoma) - 2019
- Left breast core needle biopsy (benign) - 2021

MEDICATIONS:
Home Medications:
- Ibuprofen 200-400 mg PO every 6-8 hours PRN for breast pain
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: High school science teacher.

PHYSICAL EXAMINATION:
On physical examination, the mass is smooth, mobile, and located in the upper outer quadrant of the right breast. There is no axillary lymphadenopathy.

IMAGING:
An ultrasound of the breast reveals a simple cyst measuring 2 cm in diameter. The cyst completely resolves on post-aspiration ultrasound.

OTHER STUDIES:
Fine-needle aspiration is performed, and the fluid is straw-colored and non-bloody.

## Encounter 1: source 2020-07-15, practice 2026-03-30, outpatient, General Surgery

Chief complaint: A palpable mass in her right breast that she discovered one month ago associated with tenderness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
A palpable mass in her right breast that she discovered one month ago associated with tenderness

HISTORY OF PRESENT ILLNESS:
The patient is a 42-year-old female with fibrocystic breast changes who returns to the General Surgery clinic, having presented with a right breast mass in January 2020. She describes the mass as tender to touch and denies any associated nipple discharge or skin changes. Her last menstrual period was two weeks ago. The patient expresses discomfort due to the mass and requests relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Simple breast cyst (diagnosed 2020-01-15)
- Fibrocystic breast changes

PAST SURGICAL HISTORY:
- Right breast excisional biopsy (benign fibroadenoma) - 2019
- Left breast core needle biopsy (benign) - 2021

MEDICATIONS:
Home Medications:
- Ibuprofen 200-400 mg PO every 6-8 hours PRN for breast pain
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: High school science teacher.

PHYSICAL EXAMINATION:
On physical examination, a single, mobile, smooth, and well-circumscribed mass is palpated in the upper outer quadrant of the right breast. There are no palpable axillary lymph nodes.

IMAGING:
An ultrasound of the breast is performed, revealing a round, anechoic lesion with well-defined edges and no internal echoes, measuring 2 cm in diameter. No solid components or echogenic debris are noted within the lesion.
