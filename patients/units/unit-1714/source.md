# synthetic_hospital patient 1714

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, African American, Private PPO (Aetna).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-02 (Thu) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2022-09-01 (Thu) | outpatient | Neurology |
| 2 | 2020-07-15 | 2022-11-27 (Sun) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2023-06-06 (Tue) | outpatient | Ophthalmology |
| 4 | 2021-03-15 | 2023-07-27 (Thu) | ed | Obstetrics & Gynecology |
| 5 | 2023-01-15 | 2025-05-31 (Sat) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-12-01 (Mon) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-05-30 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (Aetna)",
 "occupation": "Administrative Coordinator",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Breast cancer (diagnosed age 45)",
  "Maternal aunt: Systemic lupus erythematosus",
  "Father: Graves disease",
  "Father: Hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "400 mg PO daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Methimazole"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Tamoxifen"
  }
 ],
 "surgical_history": [
  "Cesarean section",
  "Left breast lumpectomy with sentinel lymph node biopsy"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "History of lupus nephritis",
  "Hypertension",
  "Generalized anxiety disorder",
  "Graves disease",
  "Breast cancer (status post chemotherapy, surveillance phase)",
  "Migraine with aura",
  "Adrenal Cushing syndrome"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Panic disorder", "Local Anesthetic Systemic Toxicity", "Migraine with aura", "Metastatic carcinoma to brain", "Adrenal Cushing's syndrome", "Atrial fibrillation", "Hydroxychloroquine retinopathy", "Psychogenic urinary retention due to acute stress reaction"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Generalized Anxiety Disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-06-02, outpatient, Endocrinology

Chief complaint: Progressive weight gain, fatigue, and difficulty climbing stairs over 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight gain, fatigue, and difficulty climbing stairs over 6 months

HISTORY OF PRESENT ILLNESS:
A 34-year-old woman presents to the clinic with a 6-month history of progressive weight gain, fatigue, and difficulty climbing stairs. She reports her menstrual cycles have become irregular and she has developed new facial hair.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Migraine with aura
- Adrenal Cushing syndrome

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

VITAL SIGNS:
Blood pressure is 155/95 mmHg.

PHYSICAL EXAMINATION:
On examination, she has a rounded face, supraclavicular fat pads, and a dorsocervical fat hump. Her skin appears thin with purple striae on the abdomen and easy bruising on her arms.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum cortisol (8 AM): Elevated
- ACTH: Suppressed
- Serum potassium: Low
- Fasting glucose: Elevated
- DHEAS: Normal

## Encounter 1: source 2020-04-15, practice 2022-09-01, outpatient, Neurology

Chief complaint: Recurrent severe unilateral headaches for the past 6 months with visual disturbances

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent severe unilateral headaches for the past 6 months with visual disturbances

HISTORY OF PRESENT ILLNESS:
This 34-year-old female with systemic lupus erythematosus and adrenal Cushing syndrome, recently diagnosed, presents to neurology for evaluation of headaches. She describes the headaches as severe, unilateral, and throbbing, occurring about twice a month and lasting for several hours. The headaches are often preceded by visual disturbances, including flashing lights and blind spots. She also reports sensitivity to light and sound during these episodes, as well as nausea that occasionally leads to vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Migraine with aura
- Adrenal Cushing syndrome

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
Hydrochlorothiazide, combined oral contraceptive pill

Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable, and her blood pressure is well-controlled.

## Encounter 2: source 2020-07-15, practice 2022-11-27, ed, Emergency Medicine

Chief complaint: Acute palpitations, weight loss despite increased appetite, and heat intolerance

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute palpitations, weight loss despite increased appetite, and heat intolerance

HISTORY OF PRESENT ILLNESS:
This 34-year-old with Graves disease and recent endocrine care presents to the emergency department with palpitations, weight loss despite an increased appetite, and heat intolerance over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Migraine with aura (diagnosed 2020-04-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Adrenal Cushing syndrome

She has a history of Graves' disease but has been noncompliant with her medications.

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

VITAL SIGNS:
Her heart rate is 150 beats per minute, blood pressure is 142/89 mmHg

PHYSICAL EXAMINATION:
On examination, she has a fine tremor. An irregularly irregular rhythm is noted on cardiac auscultation. Her thyroid gland is diffusely enlarged without nodularity.

LABORATORY DATA:
Thyroid function tests reveal the following:

- TSH: <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 3.2 ng/dL (normal range: 0.9-1.7 ng/dL)
- Total T3: 300 ng/dL (normal range: 80-180 ng/dL)

OTHER STUDIES:
An electrocardiogram confirms atrial fibrillation with a rapid ventricular response.

## Encounter 3: source 2021-01-15, practice 2023-06-06, outpatient, Ophthalmology

Chief complaint: Routine ophthalmology examination for medication monitoring

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Wilson, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine ophthalmology examination for medication monitoring

HISTORY OF PRESENT ILLNESS:
The patient with SLE returns for medication monitoring with migraine aura history. She reports no acute visual symptoms but mentions progressively worsened difficulty with night driving over the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Migraine with aura (diagnosed 2020-04-15)
- Atrial fibrillation (diagnosed 2020-07-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Adrenal Cushing syndrome

Her medical history is significant for hypertension, controlled with lisinopril, and a previous episode of lupus nephritis.

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
She has been on hydroxychloroquine 400 mg daily for the past 4 years.

Home Medications:
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

PHYSICAL EXAMINATION:
On examination, visual acuity is 20/20 in both eyes. Fundoscopy reveals bilateral pigmentary changes in the macula.

OTHER STUDIES:
Automated visual field testing shows bilateral central scotomas.

## Encounter 4: source 2021-03-15, practice 2023-07-27, ed, Obstetrics & Gynecology

Chief complaint: Sudden onset of confusion and agitation 2 hours after cesarean section

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of confusion and agitation 2 hours after cesarean section

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with a history of systemic lupus erythematosus, adrenal Cushing syndrome, and recent ophthalmologic monitoring for hydroxychloroquine retinopathy who presents following cesarean delivery. She is brought to the emergency department by her husband due to sudden onset of confusion and agitation. She had a cesarean section 2 hours ago under epidural anesthesia and was recovering well until she started complaining of a metallic taste in her mouth followed by perioral numbness. Her husband also reports that she had a brief episode of jerking movements involving her whole body.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Migraine with aura (diagnosed 2020-04-15)
- Atrial fibrillation (diagnosed 2020-07-15)
- Hydroxychloroquine retinopathy (diagnosed 2021-01-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Adrenal Cushing syndrome

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
She takes labetalol.

Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

VITAL SIGNS:
Blood pressure 145/95 mmHg, heart rate 102 beats/min, respiratory rate 18 breaths/min, and temperature 37.1°C (98.8°F)

PHYSICAL EXAMINATION:
She is disoriented to time and place. Her pupils are dilated but reactive to light, and she has increased muscle tone with hyperreflexia. There is no evidence of trauma.

LABORATORY DATA:
Serum sodium: 139 mEq/L
- Serum potassium: 4.1 mEq/L
- Serum chloride: 100 mEq/L
- Serum bicarbonate: 24 mEq/L
- Serum creatinine: 0.9 mg/dL
- Serum glucose: 98 mg/dL
- Complete blood count: within normal limits

## Encounter 5: source 2023-01-15, practice 2025-05-31, ed, Emergency Medicine

Chief complaint: Worsening headaches, nausea, vomiting, and transient visual disturbances over 2 weeks

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening headaches, nausea, vomiting, and transient visual disturbances over 2 weeks

HISTORY OF PRESENT ILLNESS:
This 37-year-old female with breast cancer in surveillance and prior migraine with aura presents to the emergency department with a 2-week history of worsening headaches, primarily at night, and episodes of morning nausea and vomiting. She also reports transient visual disturbances.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Migraine with aura (diagnosed 2020-04-15)
- Atrial fibrillation (diagnosed 2020-07-15)
- Hydroxychloroquine retinopathy (diagnosed 2021-01-15)
- Local Anesthetic Systemic Toxicity (diagnosed 2021-03-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Adrenal Cushing syndrome

Her medical history is significant for breast cancer, for which she completed chemotherapy 6 months ago.

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

VITAL SIGNS:
Her blood pressure is 165/95 mmHg, heart rate is 55 bpm, and respiratory rate is 14 breaths per minute.

PHYSICAL EXAMINATION:
On examination, she is alert but appears anxious. Fundoscopic examination reveals bilateral papilledema. Neurological examination shows no focal deficits.

LABORATORY DATA:
Laboratory Values:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Complete blood count (CBC): within normal limits

IMAGING:
A CT scan of the head without contrast shows a 3 cm mass lesion in the right cerebral hemisphere with surrounding edema.

## Encounter 6: source 2023-07-15, practice 2025-12-01, ed, Emergency Medicine

Chief complaint: Acute palpitations, chest tightness, and feeling of impending doom

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute palpitations, chest tightness, and feeling of impending doom

HISTORY OF PRESENT ILLNESS:
This 37-year-old with prior atrial fibrillation and recent brain metastasis presents acutely. She reports that these symptoms began suddenly while shopping at a grocery store. She has experienced similar episodes over the past 6 months, which have been increasing in frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Migraine with aura (diagnosed 2020-04-15)
- Atrial fibrillation (diagnosed 2020-07-15)
- Hydroxychloroquine retinopathy (diagnosed 2021-01-15)
- Local Anesthetic Systemic Toxicity (diagnosed 2021-03-15)
- Metastatic carcinoma to brain (diagnosed 2023-01-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Adrenal Cushing syndrome

Her past medical history is significant for generalized anxiety disorder.

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
She denies any use of caffeine, alcohol, or illicit drugs.

VITAL SIGNS:
On examination, her heart rate is 110 bpm, blood pressure is 140/85 mmHg, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
She appears anxious and is trembling.

LABORATORY DATA:
Her serum electrolytes, thyroid function tests, and complete blood count are within normal limits.

OTHER STUDIES:
An electrocardiogram shows sinus tachycardia with no acute ST changes.

## Encounter 7: source 2024-01-15, practice 2026-05-30, ed, Emergency Medicine

Chief complaint: Tingling around mouth and fingertips with acute anxiety

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Tingling around mouth and fingertips with acute anxiety

HISTORY OF PRESENT ILLNESS:
The patient is a 38-year-old female with systemic lupus erythematosus, breast cancer on surveillance following prior treatment for brain metastases, and generalized anxiety disorder, who returns to the emergency department six months after a prior visit for similar symptoms. She presents with complaints of tingling around her mouth and in her fingertips. She has a history of anxiety and reports that she has been experiencing recurrent panic attacks over the past week. During these episodes, she feels short of breath and has a sensation of impending doom, which prompts her to breathe rapidly and deeply for several minutes.

PAST MEDICAL HISTORY:
Active Problem List:
- Adrenal Cushing's syndrome (diagnosed 2020-01-15)
- Migraine with aura (diagnosed 2020-04-15)
- Atrial fibrillation (diagnosed 2020-07-15)
- Hydroxychloroquine retinopathy (diagnosed 2021-01-15)
- Local Anesthetic Systemic Toxicity (diagnosed 2021-03-15)
- Metastatic carcinoma to brain (diagnosed 2023-01-15)
- Panic disorder (diagnosed 2023-07-15)
- Systemic lupus erythematosus
- History of lupus nephritis
- Hypertension
- Generalized anxiety disorder
- Graves disease
- Breast cancer (status post chemotherapy, surveillance phase)
- Adrenal Cushing syndrome

PAST SURGICAL HISTORY:
- Cesarean section
- Left breast lumpectomy with sentinel lymph node biopsy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 400 mg PO daily
- Lisinopril 10 mg PO daily
- Sertraline 50 mg PO daily
- Methimazole 10 mg PO daily
- Tamoxifen 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Graves disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Coordinator.

VITAL SIGNS:
Her respiratory rate is 22 breaths per minute,
Her heart rate is 98 beats per minute, and blood pressure is 132/86 mmHg.

PHYSICAL EXAMINATION:
On examination, she is alert and oriented, but appears anxious.
and she is taking deep, regular breaths.
Chvostek's sign is positive.

LABORATORY DATA:
Laboratory results are as follows:

- pH: 7.52
- pCO2: 28 mmHg
- HCO3-: 24 mEq/L
- Total calcium: 9.2 mg/dL
- Ionized calcium: 3.8 mg/dL (normal 4.5-5.6 mg/dL)
