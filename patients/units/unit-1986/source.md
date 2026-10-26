# synthetic_hospital patient 1986

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-14 (Mon) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2021-12-20 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-10-15 | 2022-03-15 (Tue) | outpatient | Internal Medicine |
| 3 | 2021-04-15 | 2022-09-06 (Tue) | follow_up | Internal Medicine |
| 4 | 2021-10-15 | 2023-03-07 (Tue) | outpatient | Internal Medicine |
| 5 | 2023-01-15 | 2024-06-09 (Sun) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-06-02 (Mon) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-06-16 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "ACE inhibitors (cough intolerance)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Construction site supervisor",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Brother with melanoma",
  "Father with type 2 diabetes mellitus",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Wide local excision of nodular melanoma, upper back",
  "Chest tube placement for parapneumonic effusion",
  "Craniotomy with debulking of intracranial mass"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Hyperlipidemia",
  "History of gout"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Acute Gout (Gouty Arthritis)", "Nodular melanoma", "Uncomplicated parapneumonic effusion", "Glioblastoma (grade IV astrocytoma)", "Lisinopril adverse effect"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-06-14, outpatient, Dermatology

Chief complaint: Changing lesion on upper back noted over the past 6 months during routine skin examination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Changing lesion on upper back noted over the past 6 months during routine skin examination

HISTORY OF PRESENT ILLNESS:
He is concerned because he has a family history of skin cancer. The lesion is located on his upper back, an area that often remains uncovered during his frequent outdoor activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

His medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
He takes lisinopril, metformin.

Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
Family history of skin cancer

SOCIAL HISTORY:
Frequent outdoor activities

REVIEW OF SYSTEMS:
He denies any fever, weight loss, or night sweats.

PHYSICAL EXAMINATION:
On physical examination, you note a 1 cm pigmented lesion with an asymmetrical shape, irregular borders, and varying colors ranging from brown to black. There is no palpable lymphadenopathy.

OTHER STUDIES:
Dermoscopy shows irregular streaks and a blue-white veil.

## Encounter 1: source 2020-07-15, practice 2021-12-20, outpatient, Internal Medicine

Chief complaint: Persistent dry cough for three weeks, worse at night, without fever or respiratory infection symptoms

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Wilson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent dry cough for three weeks, worse at night, without fever or respiratory infection symptoms

HISTORY OF PRESENT ILLNESS:
The patient with T2DM, HTN, HLD, gout, history of melanoma presents with cough. He describes the cough as irritating and nonproductive, occurring throughout the day and worse at night. He denies fever, chills, chest pain, wheezing, or shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

His medical history includes type 2 diabetes mellitus diagnosed eight years ago and hypertension diagnosed five years ago.

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
He was previously treated with dietary modifications and metformin, but two months ago his primary care physician started him on a new medication for better blood pressure control and kidney protection.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
He does not smoke or drink alcohol.

VITAL SIGNS:
On examination, temperature is 36.8°C (98.2°F), blood pressure is 136/88 mmHg, and pulse is 78/min.

PHYSICAL EXAMINATION:
Cardiopulmonary examination is normal.

LABORATORY DATA:
Urinalysis shows moderate proteinuria. Laboratory testing shows normal serum creatinine and potassium levels.

## Encounter 2: source 2020-10-15, practice 2022-03-15, outpatient, Internal Medicine

Chief complaint: Follow-up visit to discuss enrollment in clinical trial for novel antihypertensive medication

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Maria Garcia, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit to discuss enrollment in clinical trial for novel antihypertensive medication

HISTORY OF PRESENT ILLNESS:
The patient, with Type 2 Diabetes Mellitus, hypertension, hyperlipidemia, and gout, returns three months after Lisinopril-associated cough and ten months after nodular melanoma diagnosis. He mentions reading about a new clinical trial for a novel antihypertensive drug and expresses interest in participating. He has no complaints of chest pain, shortness of breath, or edema. He inquires about the potential benefits and risks of joining the clinical trial.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
He takes currently on metformin and lisinopril.

Home Medications:
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
His blood pressure today is 138/86 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
His most recent laboratory values are as follows:

- Fasting glucose: 146 mg/dL
- HbA1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- LDL cholesterol: 98 mg/dL
- Potassium: 4.5 mEq/L

ASSESSMENT AND PLAN:
After reviewing the trial information, you notice that participants are assigned to treatment groups based on their preference rather than randomization.

## Encounter 3: source 2021-04-15, practice 2022-09-06, follow_up, Internal Medicine

Chief complaint: Six-month follow-up visit for ongoing clinical trial participation

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Maria Garcia, MD
Department: Internal Medicine
Visit Type: follow_up

CHIEF COMPLAINT:
Six-month follow-up visit for ongoing clinical trial participation

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with a history of nodular melanoma and prior lisinopril-induced cough returning for six-month clinical trial follow-up. He is enrolled in a randomized controlled trial to evaluate the efficacy of a new antihypertensive drug. He has been compliant with the study protocol for 6 months. His blood pressure is down from 154/96 mmHg at the start of the study. He reports strict adherence to the study medication and his usual antihypertensive regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
His other medications include metformin and atorvastatin.

Home Medications:
- Lisinopril 20 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
Blood pressure is 128/82 mmHg

LABORATORY DATA:
His laboratory values are as follows:

- Fasting glucose: 98 mg/dL
- HbA1c: 6.8%
- Total cholesterol: 180 mg/dL
- LDL cholesterol: 95 mg/dL
- Serum creatinine: 1.0 mg/dL
- Potassium: 4.5 mEq/L

ASSESSMENT AND PLAN:
The study's preliminary results show a significant reduction in blood pressure among participants who completed the study without deviation from the protocol.

## Encounter 4: source 2021-10-15, practice 2023-03-07, outpatient, Internal Medicine

Chief complaint: Final follow-up visit after completing randomized controlled trial participation

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Maria Garcia, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Final follow-up visit after completing randomized controlled trial participation

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with history of nodular melanoma, Type 2 Diabetes Mellitus, hypertension, hyperlipidemia, and gout returns for final follow-up after completing the clinical trial. He reports feeling generally well and has been compliant with his medication regimen. The patient mentions that several participants dropped out of the study because they experienced side effects from the medications, but he does not know which group they were in. The final study results suggest that there is no significant difference in blood pressure control between the two medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
Laboratory results are as follows:

- Fasting blood glucose: 138 mg/dL
- HbA1c: 7.2%
- Serum creatinine: 1.1 mg/dL
- LDL cholesterol: 98 mg/dL

## Encounter 5: source 2023-01-15, practice 2024-06-09, ed, Emergency Medicine

Chief complaint: Severe throbbing pain, redness, and swelling in right great toe for 2 days, beginning suddenly at night

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Jennifer Lee, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe throbbing pain, redness, and swelling in right great toe for 2 days, beginning suddenly at night

HISTORY OF PRESENT ILLNESS:
This patient with T2DM, HTN, hyperlipidemia, gout, and antihypertensive clinical trial participation presents with 2 days of right great toe pain. He describes throbbing pain beginning suddenly in the middle of the night and denies recent trauma or injury to foot.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

His medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
He takes hydrochlorothiazide, metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
On physical examination, the first metatarsophalangeal joint is erythematous, warm, and exquisitely tender to palpation. The range of motion is limited due to pain. The remainder of the examination is unremarkable.

LABORATORY DATA:
Synovial fluid analysis from the affected joint reveals the following:

- WBC: 18,000/mm3 (with 75% neutrophils)
- Negative for bacteria
- Crystals: Needle-shaped, negatively birefringent under polarized light

## Encounter 6: source 2024-01-15, practice 2025-06-02, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath and non-productive cough over one week, worsening when supine, following recent pneumonia

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and non-productive cough over one week, worsening when supine, following recent pneumonia

HISTORY OF PRESENT ILLNESS:
This 59-year-old male with Type 2 Diabetes Mellitus, Hypertension, Hyperlipidemia, and gout experienced a Lisinopril-associated cough in 2020. He reports feeling increasingly fatigued and says his breathing worsens when lying flat. He has no history of smoking or recent infection, but has been recovering from a left-sided pneumonia diagnosed two weeks ago, for which he completed a course of oral antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-07-15)
- Acute Gout (Gouty Arthritis) (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

His past medical history includes hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
No history of smoking

REVIEW OF SYSTEMS:
He denies fever, chest pain, or hemoptysis.

PHYSICAL EXAMINATION:
On physical examination, he appears mildly tachypneic but is hemodynamically stable. There is decreased breath sounds and dullness to percussion over the left lower lung field, with reduced tactile fremitus in the same area.

IMAGING:
Chest x-ray reveals a moderate left-sided pleural effusion, occupying nearly one-third of the hemithorax.

PLAN:
The decision is made to perform a diagnostic thoracentesis at the left midaxillary line to determine the nature of the effusion. To minimize risk of complications, the resident is instructed to insert the needle at the correct intercostal level and position relative to the rib.

## Encounter 7: source 2025-01-15, practice 2026-06-16, ed, Emergency Medicine

Chief complaint: Progressive headache, nausea, vomiting, confusion, and new right-sided weakness evolving over two months

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive headache, nausea, vomiting, confusion, and new right-sided weakness evolving over two months

HISTORY OF PRESENT ILLNESS:
The patient is a 60-year-old male with type 2 diabetes, hypertension, hyperlipidemia, and gout seen here one year ago. He presents to the emergency department with a 2-month history of progressive headache, nausea, and vomiting. He also reports episodes of confusion and a recent onset of right-sided weakness. His wife mentions that he has been increasingly lethargic and irritable.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-07-15)
- Acute Gout (Gouty Arthritis) (diagnosed 2023-01-15)
- Uncomplicated parapneumonic effusion (diagnosed 2024-01-15)
- Type 2 Diabetes Mellitus
- Hypertension
- Hyperlipidemia
- History of gout

His past medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Wide local excision of nodular melanoma, upper back
- Chest tube placement for parapneumonic effusion
- Craniotomy with debulking of intracranial mass

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily

ALLERGIES:
ACE inhibitors (cough intolerance), Sulfa drugs (rash)

FAMILY HISTORY:
- Brother with melanoma
- Father with type 2 diabetes mellitus
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
On physical examination, he is alert but appears fatigued. Neurological examination reveals right hemiparesis and a positive right Babinski sign.

IMAGING:
A non-contrast head CT shows a large left hemispheric mass with midline shift. An MRI of the brain with contrast is performed and demonstrates a heterogenous mass with serpiginous contrast enhancement and central necrosis that crosses the corpus callosum, giving a "butterfly" appearance.
