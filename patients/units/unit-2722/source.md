# synthetic_hospital patient 2722

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, African American, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-14 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2021-09-02 (Thu) | outpatient | Endocrinology |
| 2 | 2020-07-15 | 2021-12-02 (Thu) | ed | Obstetrics & Gynecology |
| 3 | 2021-01-15 | 2022-06-13 (Mon) | outpatient | Internal Medicine |
| 4 | 2021-07-15 | 2022-12-12 (Mon) | outpatient | Neurology |
| 5 | 2022-07-15 | 2023-12-14 (Thu) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2024-11-28 (Thu) | icu | Emergency Medicine |
| 7 | 2024-07-15 | 2025-12-05 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with breast cancer (diagnosed age 58)",
  "Father with hypertension and cerebral aneurysm",
  "Sister with hypothyroidism"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Spironolactone"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "500 mg PO PRN headache",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy (for papillary thyroid carcinoma)",
  "Uterine artery embolization (for symptomatic uterine fibroids)"
 ],
 "chronic_conditions": [
  "Primary hyperaldosteronism",
  "Resistant hypertension (secondary to primary hyperaldosteronism)",
  "History of papillary thyroid carcinoma",
  "Intracranial neoplasm (frontal lobe mass)",
  "Uterine fibroids (status post uterine artery embolization)",
  "Palpable right breast mass under evaluation"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Brain tumor (frontal lobe)", "Malignant neoplasm of thyroid gland", "Primary hyperaldosteronism", "Palpable breast mass, right breast", "Ruptured cerebral aneurysm", "Post-embolization syndrome", "Structural intracranial lesion"]
Comorbidities: ["Situational anxiety", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-06-14, outpatient, Internal Medicine

Chief complaint: Elevated blood pressure detected during pre-employment physical with fatigue and muscle cramps

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Elevated blood pressure detected during pre-employment physical with fatigue and muscle cramps

HISTORY OF PRESENT ILLNESS:
She reports fatigue and frequent muscle cramps over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

PAST SURGICAL HISTORY:
- Total thyroidectomy (for papillary thyroid carcinoma)
- Uterine artery embolization (for symptomatic uterine fibroids)

MEDICATIONS:
Home Medications:
- Spironolactone 50 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 58)
- Father with hypertension and cerebral aneurysm
- Sister with hypothyroidism

SOCIAL HISTORY:
She does not smoke or drink

VITAL SIGNS:
Blood pressure today is 158/100 mm Hg, and heart rate is 76/min.

PHYSICAL EXAMINATION:
She has no edema, and physical exam is otherwise normal.

LABORATORY DATA:
Laboratory results show: sodium 146 mEq/L, potassium 3.1 mEq/L, bicarbonate 31 mEq/L, and arterial pH 7.47. Her PaCO₂ is 47 mm Hg. Plasma renin activity is undetectable, and plasma aldosterone concentration is markedly elevated.

## Encounter 1: source 2020-04-15, practice 2021-09-02, outpatient, Endocrinology

Chief complaint: Routine follow-up visit after total thyroidectomy for papillary thyroid carcinoma

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit after total thyroidectomy for papillary thyroid carcinoma

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up following total thyroidectomy for papillary thyroid carcinoma. She reports feeling well and has no complaints. The patient is eager to discuss the next steps in her management to reduce the risk of recurrence.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

She has a history of hypertension.

PAST SURGICAL HISTORY:
Total thyroidectomy for papillary thyroid carcinoma

MEDICATIONS:
She takes currently taking lisinopril.

Home Medications:
- Spironolactone 50 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
Significant for a sister with breast cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Her surgical incision is healing well, and there are no signs of infection or other complications.

LABORATORY DATA:
TSH: 15 mIU/L (normal range: 0.4-4.0 mIU/L)
Thyroglobulin: Undetectable

PATHOLOGY:
2.5 cm papillary thyroid carcinoma with vascular invasion but no lymph node involvement

## Encounter 2: source 2020-07-15, practice 2021-12-02, ed, Obstetrics & Gynecology

Chief complaint: Severe lower abdominal pain and vaginal discharge two days after uterine artery embolization

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Severe lower abdominal pain and vaginal discharge two days after uterine artery embolization

HISTORY OF PRESENT ILLNESS:
This 42-year-old female with primary hyperaldosteronism and history of papillary thyroid carcinoma presents to the emergency department two days status post uterine artery embolization for uterine fibroids. She reports that the symptoms started two days ago and have progressively worsened.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2020-04-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

Her medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
She underwent uterine artery embolization one week ago for symptomatic uterine fibroids.

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Spironolactone 50 mg PO daily
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 58)
- Father with hypertension and cerebral aneurysm
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies fever, nausea, vomiting, or diarrhea.

VITAL SIGNS:
On physical examination, her temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals a tender, enlarged uterus without rebound or guarding. Pelvic examination shows a small amount of blood-tinged vaginal discharge without foul odor.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 9,000/uL
- Hemoglobin: 12.5 g/dL
- Platelet count: 250,000/uL
- C-reactive protein: 8 mg/L (normal <5 mg/L)

## Encounter 3: source 2021-01-15, practice 2022-06-13, outpatient, Internal Medicine

Chief complaint: Palpable non-tender lump in right breast discovered one month ago

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Palpable non-tender lump in right breast discovered one month ago

HISTORY OF PRESENT ILLNESS:
The patient with primary hyperaldosteronism, resistant hypertension, history of papillary thyroid carcinoma, intracranial frontal lobe mass, and uterine fibroids s/p embolization presents with a palpable non-tender right breast lump discovered one month ago. The lump is unchanged in size. She denies nipple discharge, skin changes, or pain. She is anxious about cancer and requests further evaluation.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2020-04-15)
- Post-embolization syndrome (diagnosed 2020-07-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Total thyroidectomy (for papillary thyroid carcinoma)
- Uterine artery embolization (for symptomatic uterine fibroids)

MEDICATIONS:
Home Medications:
- Spironolactone 50 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast or ovarian cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, a firm, non-mobile mass is palpated in the upper outer quadrant of the right breast. There are no overlying skin changes or nipple retraction, and no axillary or supraclavicular lymphadenopathy is appreciated. The left breast appears normal on examination.

## Encounter 4: source 2021-07-15, practice 2022-12-12, outpatient, Neurology

Chief complaint: Three-month history of persistent headaches worse at night and when bending over

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Washington, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of persistent headaches worse at night and when bending over

HISTORY OF PRESENT ILLNESS:
The patient with primary hyperaldosteronism and prior thyroid carcinoma presents with headaches, which she describes as a dull ache that worsens at night and when she bends over. She also reports episodes of nausea and occasional vomiting, particularly in the morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2020-04-15)
- Post-embolization syndrome (diagnosed 2020-07-15)
- Palpable breast mass, right breast (diagnosed 2021-01-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

She has a past medical history of hypertension.

PAST SURGICAL HISTORY:
- Total thyroidectomy (for papillary thyroid carcinoma)
- Uterine artery embolization (for symptomatic uterine fibroids)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Spironolactone 50 mg PO daily
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 58)
- Father with hypertension and cerebral aneurysm
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she has a normal blood pressure, but exhibits mild left-sided weakness and a slight tremor in her left hand.

IMAGING:
An MRI of the brain with and without contrast is performed and reveals a 2 cm enhancing mass in the right frontal lobe with surrounding vasogenic edema.

## Encounter 5: source 2022-07-15, practice 2023-12-14, ed, Emergency Medicine

Chief complaint: Severe occipital headaches with morning nausea and transient blurry vision

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe occipital headaches with morning nausea and transient blurry vision

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old female with a known frontal lobe mass and primary hyperaldosteronism who presents to the emergency department with progression of her headache symptoms. She describes the headaches as severe, throbbing, and located in the occipital region. They occur daily, particularly in the morning, and are often accompanied by nausea and vomiting. She also reports transient episodes of blurry vision. She denies any recent head trauma or use of new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2020-04-15)
- Post-embolization syndrome (diagnosed 2020-07-15)
- Palpable breast mass, right breast (diagnosed 2021-01-15)
- Brain tumor (frontal lobe) (diagnosed 2021-07-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total thyroidectomy (for papillary thyroid carcinoma)
- Uterine artery embolization (for symptomatic uterine fibroids)

MEDICATIONS:
She takes her medications include hydrochlorothiazide and amlodipine.

Home Medications:
- Spironolactone 50 mg PO daily
- Lisinopril 10 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 58)
- Father with hypertension and cerebral aneurysm
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 142/88 mmHg, heart rate is 78 bpm, and BMI is 30 kg/m^2

PHYSICAL EXAMINATION:
Fundoscopic examination reveals bilateral papilledema. Neurological examination is otherwise unremarkable.

IMAGING:
An MRI of the brain is ordered, and the results show a mass lesion in the posterior fossa with surrounding edema.

## Encounter 6: source 2023-07-15, practice 2024-11-28, icu, Emergency Medicine

Chief complaint: Sudden-onset severe headache described as 'the worst headache of my life' after lifting heavy box

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset severe headache described as 'the worst headache of my life' after lifting heavy box

HISTORY OF PRESENT ILLNESS:
The patient with frontal lobe mass and prior severe headaches presents acutely. She reports the pain started abruptly about 2 hours ago while she was lifting a heavy box. She also complains of nausea and photophobia.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2020-04-15)
- Post-embolization syndrome (diagnosed 2020-07-15)
- Palpable breast mass, right breast (diagnosed 2021-01-15)
- Brain tumor (frontal lobe) (diagnosed 2021-07-15)
- Structural intracranial lesion (diagnosed 2022-07-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

Her past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total thyroidectomy (for papillary thyroid carcinoma)
- Uterine artery embolization (for symptomatic uterine fibroids)

MEDICATIONS:
She takes lisinopril.

Home Medications:
- Spironolactone 50 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 58)
- Father with hypertension and cerebral aneurysm
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Blood pressure of 158/94 mmHg, heart rate of 88 beats per minute

PHYSICAL EXAMINATION:
On examination, she is alert but anxious, with a stiff neck.

LABORATORY DATA:
Given the high clinical suspicion for subarachnoid hemorrhage, a lumbar puncture is performed. The results are as follows:

- Opening pressure: 28 cm H2O (normal: 5-20 cm H2O)
- Red blood cell count: 1500/mm3 in tube 1 and 1480/mm3 in tube 4
- Xanthochromia: Present
- White blood cell count: 4/mm3
- Protein: 45 mg/dL (normal: 15-45 mg/dL)
- Glucose: 60 mg/dL (normal: 45-80 mg/dL)

IMAGING:
A non-contrast CT scan of the head is performed and shows no abnormalities.

## Encounter 7: source 2024-07-15, practice 2025-12-05, icu, Emergency Medicine

Chief complaint: Sudden severe headache with nausea and vomiting while gardening, similar to prior hemorrhage

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Liu, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden severe headache with nausea and vomiting while gardening, similar to prior hemorrhage

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with frontal lobe mass and prior aneurysmal hemorrhage returns with similar symptoms. She reports the headache started abruptly about 2 hours ago while she was gardening. She also complains of nausea and vomited once at home. She denies any recent trauma or falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hyperaldosteronism (diagnosed 2020-01-15)
- Malignant neoplasm of thyroid gland (diagnosed 2020-04-15)
- Post-embolization syndrome (diagnosed 2020-07-15)
- Palpable breast mass, right breast (diagnosed 2021-01-15)
- Brain tumor (frontal lobe) (diagnosed 2021-07-15)
- Structural intracranial lesion (diagnosed 2022-07-15)
- Ruptured cerebral aneurysm (diagnosed 2023-07-15)
- Resistant hypertension (secondary to primary hyperaldosteronism)
- History of papillary thyroid carcinoma
- Intracranial neoplasm (frontal lobe mass)
- Uterine fibroids (status post uterine artery embolization)
- Palpable right breast mass under evaluation

Her past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Total thyroidectomy (for papillary thyroid carcinoma)
- Uterine artery embolization (for symptomatic uterine fibroids)

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Spironolactone 50 mg PO daily
- Lisinopril 10 mg PO daily
- Amlodipine 5 mg PO daily
- Levothyroxine 125 mcg PO daily
- Acetaminophen 500 mg PO PRN headache

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 58)
- Father with hypertension and cerebral aneurysm
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 158/94 mmHg, heart rate is 88 beats per minute, and temperature is 37.1°C (98.8°F).

PHYSICAL EXAMINATION:
On examination, she is alert but appears anxious.
Neurological examination is nonfocal, but she exhibits neck stiffness when attempting to flex her neck. Fundoscopic examination is normal.

IMAGING:
A noncontrast CT scan of the head is performed and shows no evidence of hemorrhage or mass effect.

ASSESSMENT AND PLAN:
Given the high clinical suspicion for subarachnoid hemorrhage (SAH),
