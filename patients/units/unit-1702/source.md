# synthetic_hospital patient 1702

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-21 (Wed) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2023-06-26 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2021-01-15 | 2023-12-18 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-07-15 | 2024-06-17 (Mon) | outpatient | Dermatology |
| 4 | 2022-01-15 | 2024-12-31 (Tue) | outpatient | Rheumatology |
| 5 | 2022-07-15 | 2025-06-30 (Mon) | outpatient | Dermatology |
| 6 | 2023-01-15 | 2025-12-24 (Wed) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-07-01 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Never",
 "family_history": [
  "Mother: Type 1 diabetes mellitus",
  "Mother: Hashimoto thyroiditis",
  "Sister: Vitiligo",
  "Maternal aunt: Primary adrenal insufficiency"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "112 mcg daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg every morning and 10 mg every evening",
   "name": "Hydrocortisone"
  },
  {
   "dose": "0.1 mg daily",
   "name": "Fludrocortisone"
  },
  {
   "dose": "15 mg weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Thyroidectomy",
  "Cholecystectomy"
 ],
 "chronic_conditions": [
  "Autoimmune polyendocrine syndrome type 2",
  "Hashimoto thyroiditis",
  "Graves disease",
  "Primary adrenal insufficiency",
  "Celiac disease",
  "Vitiligo",
  "Alopecia areata",
  "Rheumatoid arthritis",
  "History of iron deficiency anemia"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Hypothyroidism", "Celiac disease", "Alopecia areata", "Vitiligo", "Hashimoto thyroiditis", "Adrenal crisis", "Graves disease", "Rheumatoid arthritis"]
Comorbidities: ["Medication noncompliance", "Hypothyroidism", "Hashimoto thyroiditis"]

## Encounter 0: source 2020-01-15, practice 2022-12-21, outpatient, Endocrinology

Chief complaint: Persistent fatigue and muscle stiffness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and muscle stiffness

HISTORY OF PRESENT ILLNESS:
She reports feeling increasingly tired over the past year, despite sleeping well, and notices her hair has become coarse and brittle. She has also developed mild swelling around her eyes and experiences constipation. She denies fever, weight loss, or neck pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Autoimmune polyendocrine syndrome type 2
- Hashimoto thyroiditis
- Graves disease
- Primary adrenal insufficiency
- Celiac disease
- Vitiligo
- Alopecia areata
- Rheumatoid arthritis
- History of iron deficiency anemia

Her medical history is significant only for iron deficiency anemia in the past, treated with oral iron.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
She does not take any medications.

Home Medications:
- Levothyroxine 112 mcg daily
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

VITAL SIGNS:
Her pulse is 54/min, blood pressure is 110/70 mmHg, and BMI is 24 kg/m²

PHYSICAL EXAMINATION:
Her skin is dry and cool to the touch. Physical examination reveals a diffusely enlarged, firm, non-tender thyroid gland without nodules. Neurologic examination shows delayed relaxation of deep tendon reflexes, most notably the ankle reflex.

LABORATORY DATA:
TSH                                  18 μU/mL
Free                                  T4 0.4 ng/dL
Total T3                             60 ng/dL
Serum cholesterol             Elevated
Anti-thyroglobulin antibodies are positive.

## Encounter 1: source 2020-07-15, practice 2023-06-26, outpatient, Obstetrics & Gynecology

Chief complaint: Irregular menstrual periods and infertility

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular menstrual periods and infertility

HISTORY OF PRESENT ILLNESS:
The patient returns for management of autoimmune polyendocrine syndrome type 2 and Hashimoto thyroiditis diagnosed in January 2020, presenting with menstrual irregularity and fertility concerns. She reports a 6-month history of irregular menstrual periods, which she describes as occurring every 2-3 months. She reports fatigue, weight gain, and cold intolerance and reports attempting conception for the past year without success.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Autoimmune polyendocrine syndrome type 2
- Graves disease
- Primary adrenal insufficiency
- Celiac disease
- Vitiligo
- Alopecia areata
- Rheumatoid arthritis
- History of iron deficiency anemia

Her past medical history is significant for Hashimoto's thyroiditis, for which she was prescribed levothyroxine, but she admits to poor adherence to her medication regimen.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg daily
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, her skin is dry, and she has a delayed relaxation phase of her deep tendon reflexes.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum Thyroid-Stimulating Hormone (TSH): 12 mIU/L (normal range: 0.4-4.0 mIU/L)
- Serum Free Thyroxine (Free T4): 0.7 ng/dL (normal range: 0.9-1.7 ng/dL)
- Serum Prolactin: 29 ng/mL (normal range: 4-23 ng/mL)

## Encounter 2: source 2021-01-15, practice 2023-12-18, outpatient, Gastroenterology

Chief complaint: Intermittent abdominal pain and bloating

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent abdominal pain and bloating

HISTORY OF PRESENT ILLNESS:
With autoimmune polyendocrine syndrome type 2, she returns following endocrinology care. She reports that the symptoms are worse after eating and are occasionally accompanied by diarrhea. She denies any blood in her stool, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hypothyroidism (diagnosed 2020-07-15)
- Autoimmune polyendocrine syndrome type 2
- Graves disease
- Primary adrenal insufficiency
- Celiac disease
- Vitiligo
- Alopecia areata
- Rheumatoid arthritis
- History of iron deficiency anemia

Her past medical history is significant for hypothyroidism,.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is noncontributory.

SOCIAL HISTORY:
She does not smoke or drink alcohol.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft, non-distended, with mild tenderness in the lower quadrants without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 7,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Thyroid-stimulating hormone: 2.5 mIU/L (normal: 0.4-4.0 mIU/L)
- Serum albumin: 4.5 g/dL (normal: 3.5-5.0 g/dL)
- C-reactive protein: 2 mg/L (normal: <5 mg/L)

ASSESSMENT AND PLAN:
The patient is concerned about the possibility of having a serious gastrointestinal disease.

To address her concerns, the physician decides to conduct a study comparing the prevalence of positive celiac disease serology in patients with similar symptoms who have a family history of celiac disease versus those without a family history. The physician collects data from 100 patients with similar symptoms and family histories and performs a chi-square test.

## Encounter 3: source 2021-07-15, practice 2024-06-17, outpatient, Dermatology

Chief complaint: White patches on skin

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
White patches on skin

HISTORY OF PRESENT ILLNESS:
This 34-year-old female with autoimmune polyendocrine syndrome type 2, previously evaluated for Hashimoto thyroiditis and celiac disease, returns for dermatologic assessment. She reports that the patches are asymptomatic but have been gradually increasing in size.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hypothyroidism (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Autoimmune polyendocrine syndrome type 2
- Graves disease
- Primary adrenal insufficiency
- Vitiligo
- Alopecia areata
- Rheumatoid arthritis
- History of iron deficiency anemia

She has a past medical history of Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
She is currently taking levothyroxine.

Home Medications:
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you note several flat, white macules with distinct borders on her hands and face. There is no evidence of scaling or inflammation. Wood's lamp examination accentuates the contrast between the affected and normal skin.

LABORATORY DATA:
Laboratory tests reveal the following:
- Thyroid-stimulating hormone (TSH): 3.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Fasting glucose: 88 mg/dL (normal range: 70-99 mg/dL)
- Antinuclear antibodies (ANA): Negative
- Complete blood count (CBC): Within normal limits

## Encounter 4: source 2022-01-15, practice 2024-12-31, outpatient, Rheumatology

Chief complaint: Fatigue and joint pain

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and joint pain

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with autoimmune polyendocrine syndrome type 2, recently evaluated for vitiligo, who returns to the rheumatology clinic. She presents with a 6-month history of fatigue and joint pain. She reports that the pain is symmetrical, involving her hands and knees, and is worse in the mornings. She denies any recent infections or injuries.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hypothyroidism (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Vitiligo (diagnosed 2021-07-15)
- Autoimmune polyendocrine syndrome type 2
- Graves disease
- Primary adrenal insufficiency
- Alopecia areata
- Rheumatoid arthritis
- History of iron deficiency anemia

Her medical history is significant for hypothyroidism,.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, you note bilateral swelling and tenderness in her proximal interphalangeal and metacarpophalangeal joints.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 12.1 g/dL (Normal: 12-15.5 g/dL)
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 24 mg/L (Normal: <10 mg/L)
- Rheumatoid factor (RF): Positive
- Anti-cyclic citrullinated peptide (anti-CCP) antibodies: Positive
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L (Normal: 0.4-4.0 mIU/L)

## Encounter 5: source 2022-07-15, practice 2025-06-30, outpatient, Dermatology

Chief complaint: 3-week history of hair loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-week history of hair loss

HISTORY OF PRESENT ILLNESS:
This 35-year-old with autoimmune polyendocrine syndrome type 2 and prior vitiligo returns with hair loss. She reports finding clumps of hair on her pillow in the morning and has noticed several round, completely bald patches on her scalp. She denies any associated pain, itching, or burning.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hypothyroidism (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Vitiligo (diagnosed 2021-07-15)
- Rheumatoid arthritis (diagnosed 2022-01-15)
- Autoimmune polyendocrine syndrome type 2
- Graves disease
- Primary adrenal insufficiency
- Alopecia areata
- History of iron deficiency anemia

She has a past medical history of Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
She is currently taking levothyroxine.

Home Medications:
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, there are three well-demarcated, round areas of hair loss on the scalp without erythema, scaling, or scarring. No lymphadenopathy is palpable, and the remainder of the skin examination is normal. Nail examination shows no pitting or onycholysis.

LABORATORY DATA:
Thyroid function tests from one month ago were within normal limits.

## Encounter 6: source 2023-01-15, practice 2025-12-24, ed, Emergency Medicine

Chief complaint: Palpitations, shortness of breath, and fever

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations, shortness of breath, and fever

HISTORY OF PRESENT ILLNESS:
The patient with autoimmune polyendocrine syndrome type 2, with established Hashimoto thyroiditis and Graves disease, presents with a 2-day history of palpitations, shortness of breath, and a fever. She reports medication non-compliance for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hypothyroidism (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Vitiligo (diagnosed 2021-07-15)
- Rheumatoid arthritis (diagnosed 2022-01-15)
- Alopecia areata (diagnosed 2022-07-15)
- Autoimmune polyendocrine syndrome type 2
- Graves disease
- Primary adrenal insufficiency
- History of iron deficiency anemia

Graves' disease and was scheduled for a thyroidectomy next month.

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg daily
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

VITAL SIGNS:
Temperature 39.8°C (103.6°F), heart rate 132 beats per minute, blood pressure 155/95 mmHg, respiratory rate 28 breaths per minute

PHYSICAL EXAMINATION:
She appears agitated and confused. Her skin is warm and moist, and she has a fine tremor. A thyroid exam reveals an enlarged, non-tender gland with a bruit.

LABORATORY DATA:
Total T3: 260 ng/dL (normal: 80-180 ng/dL)
Total T4: 18 µg/dL (normal: 4.6-12 µg/dL)
TSH: <0.01 IU/mL (normal: 0.4-4.0 IU/mL)
WBC: 16,000/mm³ (normal: 4,500-11,000/mm³)

OTHER STUDIES:
ECG shows atrial fibrillation with a rapid ventricular response.

## Encounter 7: source 2023-07-15, practice 2026-07-01, ed, Emergency Medicine

Chief complaint: Severe abdominal pain and vomiting

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain and vomiting

HISTORY OF PRESENT ILLNESS:
The patient with APS type 2 and history of Graves disease presents with severe abdominal pain and vomiting. She reports being unable to take her regular medications for the past two days due to nausea. She is scheduled for cholecystectomy later today due to symptomatic cholelithiasis.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Hypothyroidism (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Vitiligo (diagnosed 2021-07-15)
- Rheumatoid arthritis (diagnosed 2022-01-15)
- Alopecia areata (diagnosed 2022-07-15)
- Graves disease (diagnosed 2023-01-15)
- Autoimmune polyendocrine syndrome type 2
- Primary adrenal insufficiency
- History of iron deficiency anemia

PAST SURGICAL HISTORY:
- Thyroidectomy
- Cholecystectomy

MEDICATIONS:
Home Medications:
- Levothyroxine 112 mcg daily
- Hydrocortisone 20 mg every morning and 10 mg every evening
- Fludrocortisone 0.1 mg daily
- Methotrexate 15 mg weekly
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Vitiligo
- Maternal aunt: Primary adrenal insufficiency

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Never. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, and she is febrile with a temperature of 38.5°C (101.3°F)

PHYSICAL EXAMINATION:
Diffuse abdominal tenderness without rebound or guarding

LABORATORY DATA:
- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.5 mEq/L (Normal: 3.5-5.0 mEq/L)
- Blood glucose: 55 mg/dL (Normal: 70-99 mg/dL)
- Serum cortisol (8 AM): 3 µg/dL (Normal: 7-25 µg/dL)

ASSESSMENT AND PLAN:
Despite receiving 2 liters of normal saline, her hypotension persists.
