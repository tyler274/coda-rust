# synthetic_hospital patient 2738

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-07-25 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2023-01-23 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2023-07-20 (Thu) | outpatient | Endocrinology |
| 3 | 2021-07-15 | 2024-01-15 (Mon) | outpatient | Endocrinology |
| 4 | 2022-01-15 | 2024-07-12 (Fri) | outpatient | Endocrinology |
| 5 | 2023-01-15 | 2025-07-23 (Wed) | outpatient | Obstetrics & Gynecology |
| 6 | 2023-07-15 | 2026-01-22 (Thu) | outpatient | Neurology |
| 7 | 2024-01-15 | 2026-07-15 (Wed) | outpatient | Orthopedics |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Marketing Manager",
 "alcohol_use": "None",
 "family_history": [
  "Mother with Hashimoto thyroiditis",
  "Sister with type 1 diabetes mellitus",
  "Maternal aunt with breast cancer (diagnosed age 45)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg PO every morning, 10 mg PO every evening",
   "name": "Hydrocortisone"
  },
  {
   "dose": "0.1 mg PO daily",
   "name": "Fludrocortisone"
  },
  {
   "dose": "0.5 mg PO twice weekly",
   "name": "Cabergoline"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Tamoxifen"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Left breast lumpectomy with sentinel lymph node biopsy",
  "Transsphenoidal resection of pituitary macroadenoma"
 ],
 "chronic_conditions": [
  "Hashimoto thyroiditis",
  "Primary adrenal insufficiency",
  "Celiac disease",
  "Prolactinoma",
  "Invasive ductal carcinoma of the left breast",
  "Restless legs syndrome",
  "Hypothyroidism",
  "Morton neuroma"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Hypothyroidism", "Primary adrenal insufficiency", "Hashimoto thyroiditis", "Celiac disease", "Restless Legs Syndrome", "Invasive ductal carcinoma", "Prolactinoma", "Morton neuroma"]
Comorbidities: ["Hypothyroidism", "Hashimoto thyroiditis"]

## Encounter 0: source 2020-01-15, practice 2022-07-25, outpatient, Endocrinology

Chief complaint: Several months of progressively worsening fatigue, muscle weakness, and weight gain despite no changes in diet

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of progressively worsening fatigue, muscle weakness, and weight gain despite no changes in diet

HISTORY OF PRESENT ILLNESS:
She also reports feeling unusually cold, having difficulty concentrating at work, and experiencing persistent low mood. Her menstrual periods have become heavier and more irregular. She denies recent illness or neck pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis
- Primary adrenal insufficiency
- Celiac disease
- Prolactinoma
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Hypothyroidism
- Morton neuroma

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Sister with type 1 diabetes mellitus
- Maternal aunt with breast cancer (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

VITAL SIGNS:
Vital signs reveal a heart rate of 54/min and blood pressure of 112/70 mm Hg.

PHYSICAL EXAMINATION:
On examination, she appears pale and has dry, coarse skin with thin, brittle hair.
Neurologic examination demonstrates 4/5 strength in the proximal muscles of both upper and lower extremities without tenderness.

LABORATORY DATA:
Laboratory studies show:
Sodium                                                          130 mEq/L
Creatine kinase                                              Elevated
AST and ALT                                                  Mildly elevated
TSH                                                               Markedly elevated
Free T4                                                          Low

## Encounter 1: source 2020-07-15, practice 2023-01-23, outpatient, Gastroenterology

Chief complaint: 6-month history of chronic diarrhea, bloating, and a 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of chronic diarrhea, bloating, and a 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient with history of Hashimoto thyroiditis diagnosed in January 2020 presents with gastrointestinal complaints. She reports that her symptoms are worse after eating bread and pasta. She denies any blood in her stool or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Primary adrenal insufficiency
- Celiac disease
- Prolactinoma
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Hypothyroidism
- Morton neuroma

Her past medical history is significant for Hashimoto's thyroiditis.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Sister with type 1 diabetes mellitus
- Maternal aunt with breast cancer (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On physical examination, she appears thin and pale. Her abdominal examination reveals mild diffuse tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 400 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 15 ng/mL (Normal: 15-200 ng/mL)
- Folate: 4 ng/mL (Normal: 3-17 ng/mL)
- Vitamin B12: 220 pg/mL (Normal: 200-900 pg/mL)
- Anti-tissue transglutaminase IgA: Positive

OTHER STUDIES:
The patient undergoes a D-xylose absorption test, which shows decreased absorption.

## Encounter 2: source 2021-01-15, practice 2023-07-20, outpatient, Endocrinology

Chief complaint: Irregular menstrual cycles and difficulty conceiving

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Irregular menstrual cycles and difficulty conceiving

HISTORY OF PRESENT ILLNESS:
This 43-year-old female with Hashimoto thyroiditis (diagnosed January 2020) and celiac disease (July 2020) returns to the Endocrinology clinic. She presents with a 6-month history of irregular menstrual cycles and difficulty conceiving. She reports a decreased libido and has noticed a gradual increase in facial hair. She denies headaches, visual changes, or galactorrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Primary adrenal insufficiency
- Prolactinoma
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Hypothyroidism
- Morton neuroma

Her past medical history is significant for hypothyroidism, for which she takes levothyroxine.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Sister with type 1 diabetes mellitus
- Maternal aunt with breast cancer (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Bimanual pelvic examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum prolactin: 35 ng/mL (Normal: 4-23 ng/mL)
- Serum TSH: 5.5 µIU/mL (Normal: 0.4-4.0 µIU/mL)
- Free T4: 0.7 ng/dL (Normal: 0.9-1.7 ng/dL)
- LH: 1.2 mIU/mL (Normal: 1-12 mIU/mL, varies with menstrual cycle)
- FSH: 2.1 mIU/mL (Normal: 1.5-12.4 mIU/mL, varies with menstrual cycle)
- Estradiol: 20 pg/mL (Normal: 30-400 pg/mL, varies with menstrual cycle)

PLAN:
Magnetic resonance imaging (MRI) of the brain with a focus on the sellar region is ordered.

## Encounter 3: source 2021-07-15, practice 2024-01-15, outpatient, Endocrinology

Chief complaint: Fatigue, weight loss, and worsening skin darkening over the past several months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weight loss, and worsening skin darkening over the past several months

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with Hashimoto thyroiditis, celiac disease, and prolactinoma who returns to endocrinology with worsening fatigue and weight loss. She reports low energy levels that are interfering with her job and has lost about 6.8 kg (15 lb) unintentionally despite a normal appetite. She also notes increasing episodes of dizziness, especially when standing, and occasional salt cravings. She denies fever, cough, abdominal pain, vomiting, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Prolactinoma (diagnosed 2021-01-15)
- Primary adrenal insufficiency
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Hypothyroidism
- Morton neuroma

Her past medical history is notable for hypothyroidism.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
Levothyroxine

Home Medications:
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history includes a younger sister with vitiligo and a mother with Hashimoto thyroiditis.

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use recreational drugs. Her menstrual cycles are regular, and she is not on hormonal contraception.

VITAL SIGNS:
Blood pressure is 90/58 mm Hg supine and drops to 76/44 mm Hg upon standing. Pulse is 96/min.

PHYSICAL EXAMINATION:
On physical exam, she appears thin and slightly fatigued but is in no acute distress. Mucous membranes appear dry. She has diffuse hyperpigmentation, especially along her knuckles, elbows, and buccal mucosa. Lung and cardiac exams are normal. Abdominal exam is soft and nontender.

LABORATORY DATA:
Sodium 129 mEq/L Potassium 5.6 mEq/L Glucose 62 mg/dL BUN 20 mg/dL Creatinine 1.1 mg/dL CBC Normal TSH 3.0 mIU/L

## Encounter 4: source 2022-01-15, practice 2024-07-12, outpatient, Endocrinology

Chief complaint: Increasing fatigue, muscle aches, and difficulty climbing stairs

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, muscle aches, and difficulty climbing stairs

HISTORY OF PRESENT ILLNESS:
The patient with longstanding Hashimoto thyroiditis, primary adrenal insufficiency, and prolactinoma returns to the Endocrinology clinic for ongoing management, presenting with a 6-month history of increasing fatigue, muscle aches, and difficulty climbing stairs. She reports a weight gain of 10 pounds despite no significant change in diet or exercise. She also complains of feeling cold all the time and having dry skin.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Prolactinoma (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2021-07-15)
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Hypothyroidism
- Morton neuroma

Her past medical history is significant for Hashimoto's thyroiditis, for which she was taking levothyroxine, but she ran out of her medication several months ago and has not refilled her prescription.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Sister with type 1 diabetes mellitus
- Maternal aunt with breast cancer (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On physical examination, her heart rate is 58 bpm, and her skin is cool and dry. Deep tendon reflexes are delayed, especially at the Achilles tendon.

LABORATORY DATA:
Laboratory results are as follows:

- Serum creatine kinase (CK): 900 U/L (Normal: 38-174 U/L)
- Thyroid-stimulating hormone (TSH): 45 mIU/L (Normal: 0.4-4.0 mIU/L)
- Free T4: 0.5 ng/dL (Normal: 0.9-1.7 ng/dL)

## Encounter 5: source 2023-01-15, practice 2025-07-23, outpatient, Obstetrics & Gynecology

Chief complaint: Left breast pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Watson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Left breast pain

HISTORY OF PRESENT ILLNESS:
The 45-year-old woman with a history of Hashimoto thyroiditis diagnosed in 2020 and primary adrenal insufficiency presents to the gynecology clinic with a 3-month history of left breast pain. She describes the pain as sharp, localized to the upper outer quadrant, and states that it does not vary with her menstrual cycle. She denies any nipple discharge, skin changes, or palpable masses.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Prolactinoma (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2021-07-15)
- Hypothyroidism (diagnosed 2022-01-15)
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Morton neuroma

Her past medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

PHYSICAL EXAMINATION:
On physical examination, there is no skin dimpling, nipple retraction, or axillary lymphadenopathy. The breast examination is otherwise unremarkable.

IMAGING:
Her last mammogram was 18 months ago and was normal.

## Encounter 6: source 2023-07-15, practice 2026-01-22, outpatient, Neurology

Chief complaint: Difficulty falling asleep and an uncomfortable sensation in her legs at night

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Michael Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty falling asleep and an uncomfortable sensation in her legs at night

HISTORY OF PRESENT ILLNESS:
The patient, a 45-year-old female with a history of Hashimoto thyroiditis, primary adrenal insufficiency, celiac disease, prolactinoma, and recently diagnosed invasive ductal carcinoma of the left breast, presents to the Neurology clinic with difficulty falling asleep and an uncomfortable sensation in her legs at night for the past 6 months. She describes the sensation as a deep-seated urge to move her legs, accompanied by a creeping discomfort that worsens with rest and improves with movement. She denies any pain, weakness, or sensory changes in her legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Prolactinoma (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2021-07-15)
- Hypothyroidism (diagnosed 2022-01-15)
- Invasive ductal carcinoma (diagnosed 2023-01-15)
- Invasive ductal carcinoma of the left breast
- Restless legs syndrome
- Morton neuroma

Her medical history is significant for hypothyroidism, for which she takes levothyroxine.

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Sister with type 1 diabetes mellitus
- Maternal aunt with breast cancer (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
Neurological examination shows no abnormalities

LABORATORY DATA:
Her complete blood count is normal, but serum iron studies reveal the following:

- Serum iron: 50 mcg/dL (normal: 50-170 mcg/dL)
- Total iron-binding capacity (TIBC): 400 mcg/dL (normal: 250-450 mcg/dL)
- Ferritin: 15 ng/mL (normal: 15-200 ng/mL)

ASSESSMENT AND PLAN:
Based on the patient's clinical presentation and laboratory findings,

## Encounter 7: source 2024-01-15, practice 2026-07-15, outpatient, Orthopedics

Chief complaint: Intermittent numbness, aching, and burning pain in her forefoot

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Johnson, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent numbness, aching, and burning pain in her forefoot

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old female with autoimmune thyroiditis, adrenal insufficiency, celiac disease, prolactinoma, and history of invasive ductal carcinoma who presents with 6-month history of intermittent numbness, aching, and burning forefoot pain, particularly when wearing high heels or walking for long periods. She describes the pain as localized between the third and fourth toes and states that it worsens with tight shoes.

PAST MEDICAL HISTORY:
Active Problem List:
- Hashimoto thyroiditis (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-07-15)
- Prolactinoma (diagnosed 2021-01-15)
- Primary adrenal insufficiency (diagnosed 2021-07-15)
- Hypothyroidism (diagnosed 2022-01-15)
- Invasive ductal carcinoma (diagnosed 2023-01-15)
- Restless Legs Syndrome (diagnosed 2023-07-15)
- Invasive ductal carcinoma of the left breast
- Morton neuroma

PAST SURGICAL HISTORY:
- Left breast lumpectomy with sentinel lymph node biopsy
- Transsphenoidal resection of pituitary macroadenoma

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Hydrocortisone 20 mg PO every morning, 10 mg PO every evening
- Fludrocortisone 0.1 mg PO daily
- Cabergoline 0.5 mg PO twice weekly
- Tamoxifen 20 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Cholecalciferol 2000 units PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Sister with type 1 diabetes mellitus
- Maternal aunt with breast cancer (diagnosed age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Marketing Manager.

VITAL SIGNS:
Within normal limits

PHYSICAL EXAMINATION:
No erythema or swelling, but there is tenderness to palpation in the third metatarsal interspace. Lateral compression of the metatarsal heads elicits a click and reproduces her pain.

LABORATORY DATA:
Thyroid-stimulating hormone (TSH) level is 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
