# synthetic_hospital patient 1742

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-14 (Tue) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2022-12-08 (Thu) | outpatient | Dermatology |
| 2 | 2021-01-15 | 2023-06-09 (Fri) | outpatient | Internal Medicine |
| 3 | 2022-01-15 | 2024-06-11 (Tue) | outpatient | Endocrinology |
| 4 | 2022-07-15 | 2024-12-02 (Mon) | outpatient | Endocrinology |
| 5 | 2023-01-15 | 2025-06-09 (Mon) | outpatient | Dermatology |
| 6 | 2023-07-15 | 2025-12-06 (Sat) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-06-05 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Rare (1-2 drinks per month)",
 "family_history": [
  "Mother with Hashimoto thyroiditis",
  "Father with type 1 diabetes mellitus",
  "Paternal grandmother with vitiligo",
  "Maternal uncle with autoimmune adrenal insufficiency"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "112 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "Insulin pump: basal 1.1 units/hr, carb ratio 1:10",
   "name": "Insulin aspart"
  },
  {
   "dose": "20 mg PO every morning, 10 mg PO every evening",
   "name": "Hydrocortisone"
  },
  {
   "dose": "0.1 mg PO daily",
   "name": "Fludrocortisone"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hashimoto thyroiditis",
  "Type 1 diabetes mellitus",
  "Primary adrenal insufficiency (Addison disease)",
  "Vitiligo",
  "Acute intermittent porphyria"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Hashimoto thyroiditis", "Primary adrenal insufficiency", "Acute intermittent porphyria", "Painless thyroiditis", "Hypothyroidism", "Vitiligo"]
Comorbidities: ["Hashimoto thyroiditis", "Type 1 diabetes mellitus", "Hyperlipidemia", "Vagal bradycardia secondary to pneumoperitoneum", "Family history of porphyria"]

## Encounter 0: source 2020-01-15, practice 2022-06-14, outpatient, Endocrinology

Chief complaint: Six-month history of fatigue, weight gain, and constipation with difficulty conceiving and irregular menstrual cycles

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, weight gain, and constipation with difficulty conceiving and irregular menstrual cycles

HISTORY OF PRESENT ILLNESS:
She reports her menstrual cycles have become irregular and she has had difficulty conceiving for the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Vitiligo
- Acute intermittent porphyria

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for hypothyroidism in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her heart rate is 58 bpm

PHYSICAL EXAMINATION:
On physical examination, her skin is dry, and she has a diffusely enlarged, non-tender thyroid gland. Her reflexes are sluggish.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): 8.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (FT4): 0.7 ng/dL (normal range: 0.9-1.7 ng/dL)
- Anti-thyroid peroxidase (anti-TPO) antibodies: 400 IU/mL (normal range: <35 IU/mL)

## Encounter 1: source 2020-07-15, practice 2022-12-08, outpatient, Dermatology

Chief complaint: Six-month history of progressive depigmentation of skin starting on hands

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive depigmentation of skin starting on hands

HISTORY OF PRESENT ILLNESS:
The patient returns for dermatologic evaluation six months following her endocrinology encounter for recently diagnosed Hashimoto thyroiditis, with ongoing management of Type 1 diabetes mellitus and primary adrenal insufficiency. She presents with a 6-month history of progressive depigmentation of her skin. She first noticed small, white patches on her hands that have gradually increased in size and number. She reports no itching, pain, or discomfort in the affected areas.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Vitiligo
- Acute intermittent porphyria

Her past medical history is significant for Hashimoto's thyroiditis.

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with type 1 diabetes mellitus
- Paternal grandmother with vitiligo
- Maternal uncle with autoimmune adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you observe several well-demarcated, depigmented macules on her hands, face, and neck. Wood's lamp examination accentuates the contrast between the depigmented and normal skin. There are no other skin changes or rashes.

LABORATORY DATA:
Her thyroid-stimulating hormone (TSH) level is within normal limits.

## Encounter 2: source 2021-01-15, practice 2023-06-09, outpatient, Internal Medicine

Chief complaint: Continued fatigue, weight gain, cold intolerance, and irregular menstrual cycles

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Watson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Continued fatigue, weight gain, cold intolerance, and irregular menstrual cycles

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of Hashimoto thyroiditis and vitiligo. She reports her menstrual cycles have become irregular and she feels cold all the time.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Vitiligo (diagnosed 2020-07-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Acute intermittent porphyria

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for hypothyroidism in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, her skin is dry, and she has a delayed relaxation phase of her deep tendon reflexes. Her thyroid gland is non-tender and not enlarged.

LABORATORY DATA:
Laboratory tests reveal:

- Serum TSH: 9.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 0.7 ng/dL (normal range: 0.8-1.8 ng/dL)
- Total cholesterol: 245 mg/dL (normal range: <200 mg/dL)
- LDL cholesterol: 160 mg/dL (normal range: <100 mg/dL)

## Encounter 3: source 2022-01-15, practice 2024-06-11, outpatient, Endocrinology

Chief complaint: Fatigue, weight loss, hyperpigmentation of skin, increased salt craving, and dizziness upon standing

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weight loss, hyperpigmentation of skin, increased salt craving, and dizziness upon standing

HISTORY OF PRESENT ILLNESS:
A 36-year-old female returns to Endocrinology with established Hashimoto thyroiditis, Type 1 diabetes, and vitiligo, previously evaluated for hypothyroidism. She presents with fatigue, weight loss, and hyperpigmentation of her skin noticed over the past few months. She reports a recent increase in salt craving and has been experiencing episodes of dizziness upon standing.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Vitiligo (diagnosed 2020-07-15)
- Hypothyroidism (diagnosed 2021-01-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Acute intermittent porphyria

Her medical history is significant for autoimmune thyroiditis.

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with type 1 diabetes mellitus
- Paternal grandmother with vitiligo
- Maternal uncle with autoimmune adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Blood pressure is 90/60 mmHg

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 90/60 mmHg, and she has orthostatic hypotension. Her skin shows diffuse hyperpigmentation, most prominently in the creases of her palms and on her gums.

LABORATORY DATA:
Laboratory tests reveal the following:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- Chloride: 95 mEq/L (Normal: 98-107 mEq/L)
- Bicarbonate: 22 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 21 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Random cortisol: 2 µg/dL (Normal: 10-20 µg/dL at 8 AM)
- ACTH: Elevated

ASSESSMENT AND PLAN:
Based on the patient's presentation and laboratory findings,

## Encounter 4: source 2022-07-15, practice 2024-12-02, outpatient, Endocrinology

Chief complaint: Persistent fatigue, weight gain, constipation, and cold intolerance

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Park, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, weight gain, constipation, and cold intolerance

HISTORY OF PRESENT ILLNESS:
The patient with autoimmune polyendocrine syndrome returns for follow-up. She reports a 6-month history of fatigue, weight gain, and constipation. She reports feeling cold all the time, even in warm environments. She denies any recent changes in diet, medication, or activity level.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Vitiligo (diagnosed 2020-07-15)
- Hypothyroidism (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2022-01-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Acute intermittent porphyria

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with type 1 diabetes mellitus
- Paternal grandmother with vitiligo
- Maternal uncle with autoimmune adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her heart rate is 58 bpm, and her blood pressure is 110/70 mmHg

PHYSICAL EXAMINATION:
Her skin is dry, and she has diffuse non-pitting edema

LABORATORY DATA:
Serum TSH: 12 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 0.5 ng/dL (normal range: 0.8-1.8 ng/dL)
- Total cholesterol: 245 mg/dL (normal range: <200 mg/dL)
- LDL cholesterol: 160 mg/dL (normal range: <100 mg/dL)
- HbA1c: 6.2% (normal range: <5.7% for non-diabetics)

## Encounter 5: source 2023-01-15, practice 2025-06-09, outpatient, Dermatology

Chief complaint: Six-month history of progressive depigmented patches on face and hands with fatigue and weight gain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive depigmented patches on face and hands with fatigue and weight gain

HISTORY OF PRESENT ILLNESS:
The patient returns to Dermatology for progressive depigmentation previously evaluated in 2020. She reports that the patches initially appeared on her hands and have gradually increased in size. She also complains of fatigue, weight gain, and cold intolerance.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Vitiligo (diagnosed 2020-07-15)
- Hypothyroidism (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2022-01-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Acute intermittent porphyria

Her past medical history is significant for type 1 diabetes mellitus, which is well-controlled with insulin.

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with type 1 diabetes mellitus
- Paternal grandmother with vitiligo
- Maternal uncle with autoimmune adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals sharply demarcated, depigmented macules on her hands and face. There is no evidence of rash, erythema, or scaling. Thyroid palpation is normal.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): 9.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (T4): 0.7 ng/dL (normal range: 0.9-1.7 ng/dL)
- Anti-thyroid peroxidase antibodies: Positive
- Fasting glucose: 98 mg/dL (normal range: 70-99 mg/dL)
- Hemoglobin A1c: 6.8% (normal range: 4.0-5.6%)

## Encounter 6: source 2023-07-15, practice 2025-12-06, ed, Emergency Medicine

Chief complaint: Two-month history of palpitations, intermittent tremulousness, and unintentional weight loss despite increased appetite

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-month history of palpitations, intermittent tremulousness, and unintentional weight loss despite increased appetite

HISTORY OF PRESENT ILLNESS:
37-year-old female with Hashimoto thyroiditis and Addison disease managed in endocrinology presents with unintentional weight loss of 5 kg despite increased appetite and denies any pain or discomfort in her neck.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Vitiligo (diagnosed 2020-07-15)
- Hypothyroidism (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2022-01-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Acute intermittent porphyria

MEDICATIONS:
She is.

Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with type 1 diabetes mellitus
- Paternal grandmother with vitiligo
- Maternal uncle with autoimmune adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her heart rate is 102 bpm

PHYSICAL EXAMINATION:
She has a fine tremor in her outstretched hands. Her thyroid gland is enlarged but non-tender on palpation. There is no ophthalmopathy or dermopathy noted.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (FT4): 3.0 ng/dL (normal range: 0.9-1.7 ng/dL)
- Thyroid peroxidase antibodies (TPOAb): Positive
- Thyroglobulin antibodies (TgAb): Positive
- Erythrocyte sedimentation rate (ESR): 20 mm/hr (normal range: 0-20 mm/hr)

## Encounter 7: source 2024-01-15, practice 2026-06-05, ed, Emergency Medicine

Chief complaint: Severe, diffuse, colicky abdominal pain for 12 hours with nausea and constipation

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, diffuse, colicky abdominal pain for 12 hours with nausea and constipation

HISTORY OF PRESENT ILLNESS:
This 38-year-old female with Hashimoto thyroiditis, Type 1 diabetes, Addison disease, vitiligo, and AIP presents with abdominal pain, last evaluated in July 2023. She describes the pain as colicky and reports that it has not improved with over-the-counter analgesics. She also mentions experiencing nausea and constipation but denies any vomiting or diarrhea. She denies any recent travel, sick contacts, or changes in diet. She reports feeling very tired recently, with difficulty sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Vitiligo (diagnosed 2020-07-15)
- Hypothyroidism (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2022-01-15)
- Painless thyroiditis (diagnosed 2023-07-15)
- Type 1 diabetes mellitus
- Primary adrenal insufficiency (Addison disease)
- Acute intermittent porphyria

Her past medical history is significant for similar episodes in the past, which were thought to be related to irritable bowel syndrome.

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg PO daily
- Insulin aspart Insulin pump: basal 1.1 units/hr, carb ratio 1:10
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her brother has a history of a "blood disorder" that her family never fully understood.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, her abdomen is soft but diffusely tender without rebound or guarding. Bowel sounds are normal. She appears anxious.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800/mm^3
- Platelet count: 250,000/mm^3
- Sodium: 139 mEq/L
- Potassium: 4.1 mEq/L
- Creatinine: 0.9 mg/dL
- AST: 30 U/L
- ALT: 28 U/L
- Urinary porphobilinogen: Elevated
- Urinary urobilinogen: Elevated

ASSESSMENT AND PLAN:
Given the patient's symptoms and laboratory findings,
