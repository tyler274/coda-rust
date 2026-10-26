# synthetic_hospital patient 2531

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex M, Hispanic/Latino, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-18 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-09-30 (Fri) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2023-04-12 (Wed) | outpatient | Internal Medicine |
| 3 | 2021-07-15 | 2023-10-16 (Mon) | outpatient | Dermatology |
| 4 | 2022-01-15 | 2024-04-15 (Mon) | outpatient | Dermatology |
| 5 | 2022-07-15 | 2024-10-08 (Tue) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-04-10 (Thu) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2025-10-06 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Warehouse Associate",
 "alcohol_use": "Social",
 "family_history": [
  "Schizophrenia in maternal uncle",
  "Type 2 Diabetes Mellitus in father",
  "Hypertension in mother"
 ],
 "race_ethnicity": "Hispanic/Latino",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "3 mg PO BID",
   "name": "Risperidone"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Atenolol"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Schizophrenia",
  "Type 2 Diabetes Mellitus",
  "Hypertension",
  "Chronic Kidney Disease",
  "Obesity"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Insulin resistance due to insulin receptor downregulation", "Pseudogynecomastia", "Migraine without aura", "Tinea corporis", "Oculogyric crisis", "Tinea pedis", "Acquired methemoglobinemia", "Pheochromocytoma"]
Comorbidities: ["Schizophrenia", "Obesity", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2022-04-18, outpatient, Internal Medicine

Chief complaint: Routine follow-up with 15-pound weight gain, increased thirst and nocturia over past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with 15-pound weight gain, increased thirst and nocturia over past 6 months

HISTORY OF PRESENT ILLNESS:
He reports a 15-pound weight gain over the past 6 months. He also mentions feeling increasingly thirsty and urinating more frequently, especially at night. He denies any changes in diet or activity level. His schizophrenia has been well-controlled on olanzapine for the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

MEDICATIONS:
He takes olanzapine for the past year.

Home Medications:
- Risperidone 3 mg PO BID
- Metformin 1000 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Warehouse Associate.

VITAL SIGNS:
Blood pressure is 135/85 mmHg, BMI is 30 kg/m^2

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Fasting plasma glucose: 126 mg/dL
- Hemoglobin A1c: 7.2%
- Total cholesterol: 220 mg/dL
- LDL cholesterol: 140 mg/dL
- HDL cholesterol: 35 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 1: source 2020-07-15, practice 2022-09-30, outpatient, Endocrinology

Chief complaint: Persistent headaches, excessive sweating, and palpitations for the past 3 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Kwame Asante, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent headaches, excessive sweating, and palpitations for the past 3 months

HISTORY OF PRESENT ILLNESS:
He returns following January's visit with headaches, sweating, palpitations. He mentions these symptoms often occur in episodes, sometimes associated with a sense of impending doom.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

His medical history is significant for hypertension, which has been difficult to control despite adherence to a three-drug regimen including amlodipine, hydrochlorothiazide, and atenolol.

MEDICATIONS:
Home Medications:
- Risperidone 3 mg PO BID
- Metformin 1000 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Warehouse Associate.

VITAL SIGNS:
Blood pressure is 160/100 mmHg, and heart rate is 98 bpm

PHYSICAL EXAMINATION:
An abdominal bruit is auscultated.

LABORATORY DATA:
Serum sodium: 140 mEq/L (normal: 135-145 mEq/L)
- Serum potassium: 3.2 mEq/L (normal: 3.5-5.1 mEq/L)
- Serum glucose: 98 mg/dL (normal: 70-99 mg/dL)
- Plasma metanephrines: Elevated

A 24-hour urine collection for catecholamines and their metabolites shows increased levels of metanephrines and normetanephrines.

IMAGING:
An MRI of the abdomen demonstrates a 3 cm mass in the left adrenal gland.

## Encounter 2: source 2021-01-15, practice 2023-04-12, outpatient, Internal Medicine

Chief complaint: Concerns about the appearance of his breasts with increase in size over the past year

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Concerns about the appearance of his breasts with increase in size over the past year

HISTORY OF PRESENT ILLNESS:
The patient with schizophrenia, type 2 diabetes mellitus, and chronic kidney disease returns following evaluations for insulin resistance and pheochromocytoma. He reports that over the past year, he has noticed an increase in the size of his breasts, which has caused him significant embarrassment and reluctance to remove his shirt in public. He denies any pain, nipple discharge, or recent medication changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Pheochromocytoma (diagnosed 2020-07-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

His past medical history is significant for type 2 diabetes mellitus,.

MEDICATIONS:
He takes for which he takes metformin.

Home Medications:
- Risperidone 3 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Warehouse Associate.

VITAL SIGNS:
He has a BMI of 34 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, the breasts are symmetrically enlarged without skin changes or retractions. Palpation reveals diffuse fatty tissue without a distinct firm mass beneath the areola. There is no axillary lymphadenopathy.

LABORATORY DATA:
His recent laboratory tests show:

- Fasting glucose: 146 mg/dL
- HbA1c: 7.2%
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 3: source 2021-07-15, practice 2023-10-16, outpatient, Dermatology

Chief complaint: Recurrent itching and peeling between toes persisting for several months, worse with work footwear

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Park, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent itching and peeling between toes persisting for several months, worse with work footwear

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with schizophrenia, type 2 diabetes mellitus, and obesity, recently evaluated for pseudogynecomastia, presenting for dermatology assessment. He reports symptoms worsen with tight work shoes worn for long hours and improve somewhat when barefoot on weekends. The condition began insidiously and has been waxing and waning despite over-the-counter creams.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Pheochromocytoma (diagnosed 2020-07-15)
- Pseudogynecomastia (diagnosed 2021-01-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

He has type 2 diabetes.

MEDICATIONS:
Otherwise.

Home Medications:
- Risperidone 3 mg PO BID
- Metformin 1000 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
His job requires him to use shared locker rooms and showers, and several coworkers have had similar complaints.

REVIEW OF SYSTEMS:
He has no fever, joint pain, or systemic complaints.

PHYSICAL EXAMINATION:
A photograph is shown.

OTHER STUDIES:
A skin scraping is obtained for potassium hydroxide preparation and culture.

## Encounter 4: source 2022-01-15, practice 2024-04-15, outpatient, Dermatology

Chief complaint: 2-week history of a pruritic, expanding rash on his trunk

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Sharma, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of a pruritic, expanding rash on his trunk

HISTORY OF PRESENT ILLNESS:
Patient with schizophrenia, diabetes, and obesity returns to Dermatology after tinea pedis. He reports that the rash began as a small red area that has slowly expanded outward.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Pheochromocytoma (diagnosed 2020-07-15)
- Pseudogynecomastia (diagnosed 2021-01-15)
- Tinea pedis (diagnosed 2021-07-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

The patient has a history of type 2 diabetes mellitus.

MEDICATIONS:
He takes metformin. He denies new medications.

Home Medications:
- Risperidone 3 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
He denies known allergies

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
He denies any recent travel

PHYSICAL EXAMINATION:
On physical examination, there is a well-demarcated, erythematous, annular lesion with a raised scaly border and central clearing on his back. Similar smaller lesions are noted on his forearms.

LABORATORY DATA:
His recent laboratory values are as follows:

- Fasting glucose: 146 mg/dL
- HbA1c: 7.2%
- Complete blood count: within normal limits
- HIV test: negative

OTHER STUDIES:
KOH preparation of skin scrapings from the active border of the lesion reveals septate hyphae.

## Encounter 5: source 2022-07-15, practice 2024-10-08, ed, Emergency Medicine

Chief complaint: Several sudden episodes of sustained upward deviation of both eyes lasting up to an hour, with neck contortions

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Several sudden episodes of sustained upward deviation of both eyes lasting up to an hour, with neck contortions

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old male with schizophrenia, type 2 diabetes mellitus, hypertension, chronic kidney disease, and obesity who presents to the emergency department with new symptoms. He has established longitudinal care for insulin resistance, pheochromocytoma, pseudogynecomastia, and tinea infections. Four days prior to presentation, he was started on a new oral medication to control his psychotic symptoms. He now presents with several sudden episodes of sustained upward deviation of both eyes, each episode lasting up to an hour. These attacks are accompanied by contorted posturing of the neck and trunk, but he remains afebrile and alert between episodes. There is no history of recent ocular trauma or surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Pheochromocytoma (diagnosed 2020-07-15)
- Pseudogynecomastia (diagnosed 2021-01-15)
- Tinea pedis (diagnosed 2021-07-15)
- Tinea corporis (diagnosed 2022-01-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

MEDICATIONS:
His medication list includes risperidone, which was recently increased in dosage.

Home Medications:
- Metformin 1000 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Warehouse Associate.

REVIEW OF SYSTEMS:
He denies any visual hallucinations or changes in visual acuity.

PHYSICAL EXAMINATION:
On examination, he has no facial rash, conjunctival injection, or eyelid swelling. Neurologic examination is otherwise normal, without tremor, clonus, or muscle rigidity outside of the episodic spasms.

## Encounter 6: source 2023-01-15, practice 2025-04-10, ed, Emergency Medicine

Chief complaint: Severe throbbing headache and intermittent dizziness for the past two days, worse with light and sound

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe throbbing headache and intermittent dizziness for the past two days, worse with light and sound

HISTORY OF PRESENT ILLNESS:
A 36-year-old male with schizophrenia and diabetes returns with headache, having previously had pheochromocytoma and an oculogyric crisis six months ago. He appears visibly frustrated and is pacing back and forth in the triage area. He reports that the headache is severe, throbbing, and localized to the front of his head. He also mentions that bright lights and loud noises seem to exacerbate his pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Pheochromocytoma (diagnosed 2020-07-15)
- Pseudogynecomastia (diagnosed 2021-01-15)
- Tinea pedis (diagnosed 2021-07-15)
- Tinea corporis (diagnosed 2022-01-15)
- Oculogyric crisis (diagnosed 2022-07-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

His medical history is significant for hypertension and type 2 diabetes mellitus.

MEDICATIONS:
His medications include metformin and lisinopril.

Home Medications:
- Risperidone 3 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Warehouse Associate.

VITAL SIGNS:
On physical examination, his blood pressure is 158/94 mmHg, heart rate is 102 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
He has no focal neurological deficits, and his fundoscopic exam is normal. He becomes increasingly agitated when informed about the wait time for a CT scan, raising his voice and clenching his fists.

LABORATORY DATA:
Laboratory Values:
- Glucose: 180 mg/dL
- Hemoglobin A1c: 8.2%
- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Creatinine: 1.0 mg/dL

## Encounter 7: source 2023-07-15, practice 2025-10-06, ed, Emergency Medicine

Chief complaint: Sudden onset of shortness of breath and bluish discoloration of lips and fingers for 2 hours after starting UTI medication

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Liu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of shortness of breath and bluish discoloration of lips and fingers for 2 hours after starting UTI medication

HISTORY OF PRESENT ILLNESS:
This 36-year-old with schizophrenia and chronic kidney disease, previously seen for oculogyric crisis and migraine, presents with a 2-hour history of sudden onset of shortness of breath and a bluish discoloration of his lips and fingers. He reports that he was feeling well until he took his prescribed medication for a recently diagnosed urinary tract infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Insulin resistance due to insulin receptor downregulation (diagnosed 2020-01-15)
- Pheochromocytoma (diagnosed 2020-07-15)
- Pseudogynecomastia (diagnosed 2021-01-15)
- Tinea pedis (diagnosed 2021-07-15)
- Tinea corporis (diagnosed 2022-01-15)
- Oculogyric crisis (diagnosed 2022-07-15)
- Migraine without aura (diagnosed 2023-01-15)
- Schizophrenia
- Type 2 Diabetes Mellitus
- Hypertension
- Chronic Kidney Disease
- Obesity

MEDICATIONS:
Home Medications:
- Risperidone 3 mg PO BID
- Metformin 1000 mg PO BID
- Amlodipine 10 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Atenolol 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Schizophrenia in maternal uncle
- Type 2 Diabetes Mellitus in father
- Hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Warehouse Associate.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute. His oxygen saturation is 85% on room air, which does not improve with the administration of 100% oxygen via a non-rebreather mask.

PHYSICAL EXAMINATION:
He is anxious

LABORATORY DATA:
Arterial blood gas analysis on 100% oxygen shows a PaO2 of 98 mmHg, a PaCO2 of 38 mmHg, and a methemoglobin level of 15%. His complete blood count, electrolytes, and renal function tests are within normal limits.
