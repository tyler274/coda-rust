# synthetic_hospital patient 1751

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-23 (Mon) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2023-04-24 (Mon) | outpatient | Endocrinology |
| 2 | 2020-10-15 | 2023-10-24 (Tue) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-04-15 | 2024-04-23 (Tue) | outpatient | Obstetrics & Gynecology |
| 4 | 2021-10-15 | 2024-11-02 (Sat) | ed | Emergency Medicine |
| 5 | 2021-12-15 | 2024-12-27 (Fri) | outpatient | Endocrinology |
| 6 | 2022-10-15 | 2025-10-20 (Mon) | outpatient | Internal Medicine |
| 7 | 2023-01-15 | 2026-01-29 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private health insurance",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Hypothyroidism (Hashimoto's)",
  "Father: Type 2 diabetes mellitus",
  "Paternal aunt: Type 1 diabetes mellitus",
  "Sister: Primary adrenal insufficiency (APS-2)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "22 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale 1:15 ratio with meals",
   "name": "Insulin aspart"
  },
  {
   "dose": "112 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg PO every morning, 10 mg PO every afternoon",
   "name": "Hydrocortisone"
  },
  {
   "dose": "0.1 mg PO daily",
   "name": "Fludrocortisone"
  },
  {
   "dose": "50 mg PO as needed for migraine (max 200 mg/day)",
   "name": "Sumatriptan"
  }
 ],
 "surgical_history": [
  "Vaginal delivery at 34 weeks gestation",
  "None otherwise"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus",
  "Hashimoto's thyroiditis",
  "Primary adrenal insufficiency (Addison disease)",
  "History of miliary tuberculosis",
  "Latent tuberculosis infection",
  "Temporal lobe epilepsy",
  "Migraine without aura"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Primary adrenal insufficiency", "Latent tuberculosis infection", "Painless thyroiditis", "Respiratory distress syndrome", "Temporal lobe epilepsy"]
Comorbidities: ["Type 1 diabetes mellitus", "Hashimoto thyroiditis", "Autoimmune polyglandular syndrome type 2 (Schmidt syndrome)"]

## Encounter 0: source 2020-01-15, practice 2023-01-23, outpatient, Endocrinology

Chief complaint: Increasing fatigue, unintentional weight loss of 10 kg over the past 6 months, and difficulty standing from a seated position

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, unintentional weight loss of 10 kg over the past 6 months, and difficulty standing from a seated position

HISTORY OF PRESENT ILLNESS:
She also reports a recent onset of salt craving.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Latent tuberculosis infection
- Temporal lobe epilepsy
- Migraine without aura

Her past medical history is significant for primary hypothyroidism,.

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism (Hashimoto's)
- Father: Type 2 diabetes mellitus
- Paternal aunt: Type 1 diabetes mellitus
- Sister: Primary adrenal insufficiency (APS-2)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her blood pressure is 90/60 mmHg, and she has hyperpigmentation on her knuckles, elbows, and oral mucosa.

LABORATORY DATA:
Laboratory tests reveal the following:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- Chloride: 95 mEq/L (Normal: 98-106 mEq/L)
- Bicarbonate: 22 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 28 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (Normal: 0.6-1.1 mg/dL)
- Fasting glucose: 70 mg/dL (Normal: 70-99 mg/dL)
- ACTH: Elevated
- Cortisol: Low

## Encounter 1: source 2020-04-15, practice 2023-04-24, outpatient, Endocrinology

Chief complaint: Fatigue, weight loss, and hyperpigmentation of her skin that has developed over the past few months, with morning weakness and dizziness upon standing

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weight loss, and hyperpigmentation of her skin that has developed over the past few months, with morning weakness and dizziness upon standing

HISTORY OF PRESENT ILLNESS:
She returns for follow-up three months after her prior visit with Type 1 Diabetes Mellitus and Hashimoto's thyroiditis. Hyperpigmentation has developed over the past few months. She feels particularly weak in the mornings and has occasional dizziness upon standing.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Latent tuberculosis infection
- Temporal lobe epilepsy
- Migraine without aura

Her past medical history is significant for type 1 diabetes mellitus and autoimmune thyroiditis.

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Levothyroxine 112 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism (Hashimoto's)
- Father: Type 2 diabetes mellitus
- Paternal aunt: Type 1 diabetes mellitus
- Sister: Primary adrenal insufficiency (APS-2)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 100/60 mmHg, and she has orthostatic hypotension.

PHYSICAL EXAMINATION:
Her skin examination reveals diffuse hyperpigmentation, most prominent in the creases of her palms and on her gums.

LABORATORY DATA:
Initial laboratory tests reveal the following:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- 8 AM serum cortisol: 3 µg/dL (Normal: 7-25 µg/dL)
- Plasma ACTH: Elevated

## Encounter 2: source 2020-10-15, practice 2023-10-24, outpatient, Obstetrics & Gynecology

Chief complaint: Six-month history of irregular menstrual cycles ranging from 21 to 60 days apart and difficulty conceiving

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of irregular menstrual cycles ranging from 21 to 60 days apart and difficulty conceiving

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with Type 1 Diabetes Mellitus, Hashimoto's thyroiditis, and primary adrenal insufficiency diagnosed in January 2020 presents to Obstetrics and Gynecology with menstrual irregularity and fertility concerns. She reports her menstrual cycles have become increasingly sporadic, ranging from 21 to 60 days apart, and she has not had a period for the past 3 months. She also complains of hot flashes, night sweats, and vaginal dryness.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Latent tuberculosis infection
- Temporal lobe epilepsy
- Migraine without aura

Her past medical history is significant for Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
She denies any family history of early menopause or autoimmune diseases.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her thyroid gland is non-tender and of normal size. A pelvic examination reveals no abnormalities.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum FSH: 108 mIU/mL (normal range: 3.5-12.5 mIU/mL)
- Serum LH: 78 mIU/mL (normal range: 2.4-12.6 mIU/mL)
- Serum estradiol: 18 pg/mL (normal range: 30-400 pg/mL)
- TSH: 2.0 µIU/mL (normal range: 0.4-4.0 µIU/mL)
- Free T4: 1.2 ng/dL (normal range: 0.8-1.8 ng/dL)

## Encounter 3: source 2021-04-15, practice 2024-04-23, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at approximately 10 weeks gestation

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at approximately 10 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient returns for routine prenatal care at 10 weeks gestation following prior evaluation for menstrual irregularities, with autoimmune endocrinopathies. She was recently diagnosed with latent tuberculosis (TB) after a positive tuberculin skin test and a subsequent chest X-ray that showed no active disease. She denies any symptoms of active TB, such as cough, fever, night sweats, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Latent tuberculosis infection
- Temporal lobe epilepsy
- Migraine without aura

Her past medical history is significant for type 1 diabetes mellitus, which is well-controlled on an insulin pump.

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Levothyroxine 112 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism (Hashimoto's)
- Father: Type 2 diabetes mellitus
- Paternal aunt: Type 1 diabetes mellitus
- Sister: Primary adrenal insufficiency (APS-2)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Her prenatal labs are within normal limits

PLAN:
She is started on isoniazid (INH) therapy for latent TB.

## Encounter 4: source 2021-10-15, practice 2024-11-02, ed, Emergency Medicine

Chief complaint: Active labor at 34 weeks of gestation with premature rupture of membranes

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Active labor at 34 weeks of gestation with premature rupture of membranes

HISTORY OF PRESENT ILLNESS:
The patient with Addison disease presents in labor. She has had no prenatal care. Shortly after arrival, she delivers a male infant weighing 2,100 grams.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Latent tuberculosis infection (diagnosed 2021-04-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Temporal lobe epilepsy
- Migraine without aura

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Levothyroxine 112 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism (Hashimoto's)
- Father: Type 2 diabetes mellitus
- Paternal aunt: Type 1 diabetes mellitus
- Sister: Primary adrenal insufficiency (APS-2)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
The infant's heart rate is 160 beats per minute, and the oxygen saturation is 85% on room air.

PHYSICAL EXAMINATION:
The infant has a weak cry and appears dusky. On examination, the infant is tachypneic with a respiratory rate of 70 breaths per minute, has intercostal retractions, nasal flaring, and is grunting.

IMAGING:
A chest x-ray is performed, showing a fine reticular granularity of the lungs with decreased lung volumes.

## Encounter 5: source 2021-12-15, practice 2024-12-27, outpatient, Endocrinology

Chief complaint: Two-week history of palpitations, irritability, and unintentional weight loss of 5 kg despite increased appetite

ENCOUNTER NOTE
Date: 2021-12-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of palpitations, irritability, and unintentional weight loss of 5 kg despite increased appetite

HISTORY OF PRESENT ILLNESS:
The patient with established autoimmune endocrinopathies returns two months postpartum. She denies any pain in her neck but mentions feeling a slight fullness in the throat.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Latent tuberculosis infection (diagnosed 2021-04-15)
- Respiratory distress syndrome (diagnosed 2021-10-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Temporal lobe epilepsy
- Migraine without aura

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
She does not take any medications.

Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Levothyroxine 112 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for Hashimoto's thyroiditis in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her heart rate is 102 bpm,

PHYSICAL EXAMINATION:
On examination, her thyroid is symmetrically enlarged without tenderness, and there is no overlying erythema or warmth.
and her skin is warm and moist.

LABORATORY DATA:
Laboratory tests reveal the following:
- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (FT4): 35 pmol/L (normal range: 12-22 pmol/L)
- Free triiodothyronine (FT3): 9.0 pmol/L (normal range: 3.1-6.8 pmol/L)
- Thyroid peroxidase antibodies (TPOAb): Negative

IMAGING:
Thyroid scintigraphy shows decreased radioiodine uptake.

## Encounter 6: source 2022-10-15, practice 2025-10-20, outpatient, Internal Medicine

Chief complaint: Fatigue, dizziness, and recent onset of salt craving with 10-pound weight loss over two months

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Michael Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, dizziness, and recent onset of salt craving with 10-pound weight loss over two months

HISTORY OF PRESENT ILLNESS:
The patient with primary adrenal insufficiency diagnosed in 2020 reports a 10-pound weight loss over the past two months despite no changes in diet or exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Latent tuberculosis infection (diagnosed 2021-04-15)
- Respiratory distress syndrome (diagnosed 2021-10-15)
- Painless thyroiditis (diagnosed 2021-12-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Temporal lobe epilepsy
- Migraine without aura

She has a history of miliary tuberculosis treated six months ago.

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin aspart sliding scale 1:15 ratio with meals
- Levothyroxine 112 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily
- Sumatriptan 50 mg PO as needed for migraine (max 200 mg/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism (Hashimoto's)
- Father: Type 2 diabetes mellitus
- Paternal aunt: Type 1 diabetes mellitus
- Sister: Primary adrenal insufficiency (APS-2)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg

PHYSICAL EXAMINATION:
On physical examination, she has hyperpigmentation on her elbows and knees.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.8 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 95 mEq/L (Normal: 98-107 mEq/L)
- Bicarbonate: 22 mEq/L (Normal: 22-29 mEq/L)
- Blood urea nitrogen (BUN): 28 mg/dL (Normal: 7-20 mg/dL)
- Creatinine: 1.2 mg/dL (Normal: 0.6-1.1 mg/dL)
- Random cortisol: 2 µg/dL (Normal: 10-20 µg/dL at 8 AM)

## Encounter 7: source 2023-01-15, practice 2026-01-29, ed, Emergency Medicine

Chief complaint: Sudden episode of unresponsiveness with lip smacking witnessed by husband

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Ahmed Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden episode of unresponsiveness with lip smacking witnessed by husband

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with a known history of temporal lobe epilepsy, Type 1 Diabetes Mellitus, and primary adrenal insufficiency who was evaluated three months ago for recurrent adrenal insufficiency symptoms and is brought to the emergency department by her husband after experiencing a sudden episode of unresponsiveness while cooking dinner. The husband reports that she stopped talking mid-sentence, stared blankly, and began smacking her lips repeatedly. She did not respond to her name being called and did not recall the event afterward. The episode lasted for about 2 minutes, and she appeared confused for an additional 10 minutes post-event.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary adrenal insufficiency (diagnosed 2020-01-15)
- Latent tuberculosis infection (diagnosed 2021-04-15)
- Respiratory distress syndrome (diagnosed 2021-10-15)
- Painless thyroiditis (diagnosed 2021-12-15)
- Type 1 Diabetes Mellitus
- Hashimoto's thyroiditis
- Primary adrenal insufficiency (Addison disease)
- History of miliary tuberculosis
- Temporal lobe epilepsy
- Migraine without aura

Her past medical history is significant for type 1 diabetes mellitus and migraines.

PAST SURGICAL HISTORY:
- Vaginal delivery at 34 weeks gestation
- None otherwise

MEDICATIONS:
Her medications include insulin glargine, insulin aspart, and sumatriptan as needed.

Home Medications:
- Levothyroxine 112 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every afternoon
- Fludrocortisone 0.1 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hypothyroidism (Hashimoto's)
- Father: Type 2 diabetes mellitus
- Paternal aunt: Type 1 diabetes mellitus
- Sister: Primary adrenal insufficiency (APS-2)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, she is alert and oriented but reports a mild headache and fatigue. Her neurological examination is unremarkable.

LABORATORY DATA:
Her blood glucose level at the time of the examination is 98 mg/dL.

OTHER STUDIES:
An electroencephalogram (EEG) shows focal epileptiform discharges in the left temporal lobe.
