# synthetic_hospital patient 2782

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 40 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-26 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-02-15 | 2023-08-18 (Fri) | outpatient | Internal Medicine |
| 2 | 2022-07-15 | 2026-01-07 (Wed) | follow_up | Otolaryngology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Seasonal pollen",
  "Dust mites"
 ],
 "insurance": "Private PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Maternal aunt: Papillary thyroid carcinoma",
  "Mother: Hypothyroidism",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "10 mg PO daily PRN for seasonal allergies",
   "name": "Loratadine"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy (2 years after initial encounter)",
  "Radioactive iodine ablation therapy"
 ],
 "chronic_conditions": [
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 40
}
```

Primary diagnoses: ["Malignant neoplasm of thyroid gland"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2023-07-26, outpatient, Internal Medicine

Chief complaint: Noticing a lump on the front of her neck two weeks ago while looking in the mirror

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Noticing a lump on the front of her neck two weeks ago while looking in the mirror

HISTORY OF PRESENT ILLNESS:
She denies pain, dysphagia, or changes in voice. She feels well otherwise, with no weight change, fatigue, or mood disturbance. Her menstrual cycles are regular

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Total thyroidectomy (2 years after initial encounter)
- Radioactive iodine ablation therapy

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Loratadine 10 mg PO daily PRN for seasonal allergies

ALLERGIES:
Seasonal pollen, Dust mites

FAMILY HISTORY:
No family history of thyroid cancer or endocrine disorders

SOCIAL HISTORY:
She has no history of radiation exposure

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
On examination, her thyroid is not enlarged overall, but a 2-cm firm, non-tender nodule is palpated in the right lobe. There is no cervical lymphadenopathy.

IMAGING:
A neck ultrasound shows a solid hypoechoic nodule with irregular borders

PATHOLOGY:
Fine-needle aspiration (FNA) is performed. Cytology reveals deposits that stain with Congo red and show apple-green birefringence under polarized light.

## Encounter 1: source 2020-02-15, practice 2023-08-18, outpatient, Internal Medicine

Chief complaint: Evaluation of a swelling in her neck noticed several weeks ago while adjusting her necklace

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. James Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a swelling in her neck noticed several weeks ago while adjusting her necklace

HISTORY OF PRESENT ILLNESS:
The patient returns for continued evaluation of neck swelling initially observed one month ago. She reports no pain, voice changes, difficulty breathing, or trouble swallowing solid foods or liquids. She has felt generally well with stable weight, normal appetite, and no recent illnesses.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of thyroid gland (diagnosed 2020-01-15)
- Seasonal allergic rhinitis

Her medical history includes seasonal allergies managed with over-the-counter antihistamines.

PAST SURGICAL HISTORY:
- Total thyroidectomy (2 years after initial encounter)
- Radioactive iodine ablation therapy

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Loratadine 10 mg PO daily PRN for seasonal allergies

ALLERGIES:
Seasonal pollen, Dust mites

FAMILY HISTORY:
Family history is unremarkable.

SOCIAL HISTORY:
She does not smoke and drinks alcohol socially. She works as an elementary school teacher and denies recent travel or exposure to environmental toxins.

VITAL SIGNS:
On examination, her temperature is 36.7°C (98.1°F), pulse is 74/min, and blood pressure is 118/76 mmHg. BMI is 24 kg/m².

PHYSICAL EXAMINATION:
Palpation of the anterior neck reveals a firm, non-tender, mobile nodule approximately 1.8 cm in diameter in the right thyroid lobe without cervical lymphadenopathy. The remainder of her examination is normal.

LABORATORY DATA:
Thyroid function tests are within normal limits.

PATHOLOGY:
Fine needle aspiration biopsy shows branching structures with fibrovascular cores, in addition to the structures shown.

## Encounter 2: source 2022-07-15, practice 2026-01-07, follow_up, Otolaryngology

Chief complaint: Routine follow-up visit after thyroid cancer treatment

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Tan, MD
Department: Otolaryngology
Visit Type: follow_up

CHIEF COMPLAINT:
Routine follow-up visit after thyroid cancer treatment

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up following treatment of the thyroid malignancy initially identified in January 2020 after she noted an anterior neck lump. She reports no symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of thyroid gland (diagnosed 2020-01-15)
- Seasonal allergic rhinitis

She has a history of papillary thyroid carcinoma for which she underwent total thyroidectomy and radioactive iodine ablation therapy 2 years ago.

PAST SURGICAL HISTORY:
- Total thyroidectomy (2 years after initial encounter)
- Radioactive iodine ablation therapy

MEDICATIONS:
She is currently on levothyroxine replacement therapy.

Home Medications:
- Loratadine 10 mg PO daily PRN for seasonal allergies

ALLERGIES:
Seasonal pollen, Dust mites

FAMILY HISTORY:
- Maternal aunt: Papillary thyroid carcinoma
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
- Serum thyroglobulin: 10 ng/mL (increased from <1 ng/mL six months ago)
- Thyroid-stimulating hormone (TSH): within normal limits
- Free T4: within normal limits
- Serum calcium: within normal limits
