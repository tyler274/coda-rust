# synthetic_hospital patient 2851

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-02 (Thu) | outpatient | Neurology |
| 1 | 2020-04-15 | 2023-05-01 (Mon) | outpatient | Pulmonology |
| 2 | 2020-07-15 | 2023-07-24 (Mon) | outpatient | Dermatology |
| 3 | 2021-01-15 | 2024-01-31 (Wed) | outpatient | Obstetrics & Gynecology |
| 4 | 2022-01-15 | 2025-01-24 (Fri) | outpatient | Cardiology |
| 5 | 2022-07-15 | 2025-07-17 (Thu) | outpatient | Rheumatology |
| 6 | 2023-01-15 | 2026-02-01 (Sun) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-07-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "Occasional",
 "family_history": [
  "Mother: primary open-angle glaucoma",
  "Father: hypertension",
  "Sister: rheumatoid arthritis",
  "Brother: essential tremor"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "90 mcg 2 puffs PRN",
   "name": "Albuterol inhaler"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Chronic kidney disease",
  "Rheumatoid arthritis",
  "Hyperlipidemia",
  "Asthma",
  "Essential tremor"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Hypertensive Heart Disease with End-Organ Damage (indication for echocardiography)", "Asthma", "Cervical Cancer Screening Encounter", "Cherry hemangioma", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Acute angle-closure glaucoma", "Voluntary withdrawal from clinical research protocol", "Essential tremor"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-02-02, outpatient, Neurology

Chief complaint: Bilateral hand tremors for 5 years, worse with action

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral hand tremors for 5 years, worse with action

HISTORY OF PRESENT ILLNESS:
5-year history of bilateral hand tremors. She reports that the tremors are more noticeable when she is holding objects, like a cup of coffee or writing with a pen, and they seem to improve when her hands are at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia
- Asthma
- Essential tremor

Her past medical history is significant for hypertension.

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
Her father had a similar tremor that began in his late 50s.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional. Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies any weakness, numbness, or other neurological symptoms.

PHYSICAL EXAMINATION:
On examination, you observe a fine, bilateral, postural tremor of her hands that worsens when she extends her arms and during finger-to-nose testing. There is no rigidity, bradykinesia, or other abnormal movements. Her gait is normal.

LABORATORY DATA:
Routine blood work including thyroid function tests and liver enzymes are within normal limits.

## Encounter 1: source 2020-04-15, practice 2023-05-01, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and wheezing over 6 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and wheezing over 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old female with HTN, CKD, RA, and asthma last evaluated in Neurology three months ago for tremor. She reports a 6-month history of progressive shortness of breath, especially on exertion, and a wheezing sound that she notices mostly at night. She reports that her symptoms are partially relieved by over-the-counter albuterol inhaler, which she started using a month ago on her neighbor's suggestion. She has no history of atopic disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia
- Asthma

MEDICATIONS:
She takes hydrochlorothiazide, over-the-counter albuterol inhaler.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: primary open-angle glaucoma
- Father: hypertension
- Sister: rheumatoid arthritis
- Brother: essential tremor

SOCIAL HISTORY:
She has never smoked.

PHYSICAL EXAMINATION:
On physical examination, you note a respiratory rate of 22 breaths per minute, and auscultation of the lungs reveals bilateral expiratory wheezes. No other abnormalities are noted.

OTHER STUDIES:
Spirometry shows a forced expiratory volume in 1 second (FEV1) of 65% predicted, which improves to 80% predicted after administration of a bronchodilator. The diffusion capacity for carbon monoxide (DLCO) is within normal limits.

## Encounter 2: source 2020-07-15, practice 2023-07-24, outpatient, Dermatology

Chief complaint: Multiple new red papules on trunk noticed over past year

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Elena Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple new red papules on trunk noticed over past year

HISTORY OF PRESENT ILLNESS:
This 65-year-old female with essential tremor and asthma presents to Dermatology following recent Neurology and Pulmonology care. She reports that over the past year, she has noticed multiple small, bright red papules on her trunk. She is concerned because they seem to be increasing in number. She denies any pain, itching, or bleeding from the lesions.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia

Her medical history is significant for hypertension.

MEDICATIONS:
She takes for which she takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is unremarkable

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note numerous 1-3 mm, round, non-blanching, bright red papules scattered over her chest and abdomen. There are no similar lesions on her palms, soles, or mucous membranes. The remainder of the examination is unremarkable.

LABORATORY DATA:
Complete blood count: within normal limits
- Comprehensive metabolic panel: within normal limits
- Thyroid-stimulating hormone: within normal limits

## Encounter 3: source 2021-01-15, practice 2024-01-31, outpatient, Obstetrics & Gynecology

Chief complaint: Annual wellness visit with questions about cervical cancer screening

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Annual wellness visit with questions about cervical cancer screening

HISTORY OF PRESENT ILLNESS:
She returns for annual wellness visit with essential tremor and asthma. She has no complaints, is up to date with all recommended vaccinations, and inquires about the need for continued Pap testing.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia

Her past medical history is significant for hypertension, controlled with lisinopril, and hyperlipidemia, managed with atorvastatin. Her last Pap smear 3 years ago was normal, and she has had normal Pap smears consistently every 3 years since the age of 30.

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of gynecologic cancers.

SOCIAL HISTORY:
She has never smoked and denies any history of sexually transmitted infections.

PHYSICAL EXAMINATION:
Her physical examination, including a pelvic exam, is unremarkable.

## Encounter 4: source 2022-01-15, practice 2025-01-24, outpatient, Cardiology

Chief complaint: Progressive fatigue and dyspnea with exertion over several months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and dyspnea with exertion over several months

HISTORY OF PRESENT ILLNESS:
Hypertensive patient with asthma presents with progressive fatigue and dyspnea with exertion over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Cervical Cancer Screening Encounter (diagnosed 2021-01-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia

She has a long-standing history of poorly controlled hypertension but no known history of coronary artery disease.

MEDICATIONS:
Her medications include amlodipine and hydrochlorothiazide, but she admits to missing doses regularly.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: primary open-angle glaucoma
- Father: hypertension
- Sister: rheumatoid arthritis
- Brother: essential tremor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional. Occupation: Retired administrative assistant.

VITAL SIGNS:
On physical examination, her blood pressure is 158/90 mm Hg, pulse is 82/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a sustained point of maximal impulse. Pulmonary auscultation is clear.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 5: source 2022-07-15, practice 2025-07-17, outpatient, Rheumatology

Chief complaint: Request to withdraw from clinical research study after 3 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Wilson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Request to withdraw from clinical research study after 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old female with rheumatoid arthritis, essential tremor, and hypertensive heart disease who returns to Rheumatology requesting withdrawal from a clinical research study. She previously presented to Cardiology six months ago with progressive fatigue and dyspnea with exertion. She has been participating in a Phase III clinical trial investigating a new biologic agent for inflammatory arthritis. She enrolled 3 months ago after meeting eligibility criteria and signed a detailed informed consent document. Since starting the study, she has noticed mild improvement in her joint stiffness, but she also feels increasingly fatigued and frustrated by the weekly clinic visits and frequent blood draws. The physician overseeing the study is notified and comes in to speak with her.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Cervical Cancer Screening Encounter (diagnosed 2021-01-15)
- Hypertensive Heart Disease with End-Organ Damage (indication for echocardiography) (diagnosed 2022-01-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: primary open-angle glaucoma
- Father: hypertension
- Sister: rheumatoid arthritis
- Brother: essential tremor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional. Occupation: Retired administrative assistant.

ASSESSMENT AND PLAN:
The patient does not appear distressed or confused and makes it clear that she no longer wishes to continue. She does not ask any specific questions, nor does she offer further explanation.

The physician wants to handle the matter ethically and appropriately.

## Encounter 6: source 2023-01-15, practice 2026-02-01, ed, Emergency Medicine

Chief complaint: Generalized weakness and recurrent muscle cramps for 1 week

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Khan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized weakness and recurrent muscle cramps for 1 week

HISTORY OF PRESENT ILLNESS:
68-year-old with HTN, CKD, RA, cardiology and rheumatology presents to the emergency department with complaints of generalized weakness and recurrent muscle cramps over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Cervical Cancer Screening Encounter (diagnosed 2021-01-15)
- Hypertensive Heart Disease with End-Organ Damage (indication for echocardiography) (diagnosed 2022-01-15)
- Voluntary withdrawal from clinical research protocol (diagnosed 2022-07-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia

MEDICATIONS:
She reports adherence to her medication regimen, which includes hydrochlorothiazide for blood pressure control.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: primary open-angle glaucoma
- Father: hypertension
- Sister: rheumatoid arthritis
- Brother: essential tremor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her blood pressure is 145/90 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears mildly dehydrated. Neurological examination reveals decreased muscle strength in her lower extremities and diminished deep tendon reflexes.

LABORATORY DATA:
- Serum potassium: 2.9 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum sodium: 140 mEq/L (Normal: 135-145 mEq/L)
- Serum creatinine: 1.8 mg/dL (Normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 29 mg/dL (Normal: 7-20 mg/dL)
- Serum magnesium: 1.8 mg/dL (Normal: 1.7-2.2 mg/dL)

OTHER STUDIES:
An electrocardiogram (ECG) shows U waves, flattened T waves, and occasional premature ventricular contractions.

## Encounter 7: source 2023-07-15, practice 2026-07-21, ed, Emergency Medicine

Chief complaint: Sudden severe headache, nausea, and blurred vision in right eye after movie

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe headache, nausea, and blurred vision in right eye after movie

HISTORY OF PRESENT ILLNESS:
Patient is a 68-year-old female with HTN, CKD, and asthma who returns to the ED with headache that began several hours ago after attending a movie premiere. She describes the pain as throbbing and localized around her right eye, rating it as 9 out of 10 in intensity.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Cherry hemangioma (diagnosed 2020-07-15)
- Cervical Cancer Screening Encounter (diagnosed 2021-01-15)
- Hypertensive Heart Disease with End-Organ Damage (indication for echocardiography) (diagnosed 2022-01-15)
- Voluntary withdrawal from clinical research protocol (diagnosed 2022-07-15)
- Hypokalemia managed with oral replacement (tolerating PO intake) (diagnosed 2023-01-15)
- Hypertension
- Chronic kidney disease
- Rheumatoid arthritis
- Hyperlipidemia

Her past medical history is significant for hypertension.

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Albuterol inhaler 90 mcg 2 puffs PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: primary open-angle glaucoma
- Father: hypertension
- Sister: rheumatoid arthritis
- Brother: essential tremor

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her right eye is red with conjunctival injection, and the cornea appears hazy. The pupil of the affected eye is mid-dilated and non-reactive to light. Palpation of the globe reveals a firm eyeball. Visual acuity is decreased in the right eye compared to the left.

OTHER STUDIES:
Intraocular pressure measured by tonometry is 48 mmHg in the right eye and 15 mmHg in the left eye. Gonioscopy of the right eye shows a closed angle.
