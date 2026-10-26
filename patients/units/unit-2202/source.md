# synthetic_hospital patient 2202

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-08-13 (Tue) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2019-09-30 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2020-01-27 (Mon) | outpatient | Urology |
| 3 | 2021-07-15 | 2021-02-09 (Tue) | outpatient | Neurology |
| 4 | 2023-01-15 | 2022-08-10 (Wed) | outpatient | Cardiology |
| 5 | 2025-01-15 | 2024-08-01 (Thu) | ed | Emergency Medicine |
| 6 | 2026-01-15 | 2025-08-03 (Sun) | ed | Emergency Medicine |
| 7 | 2027-01-15 | 2026-08-02 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "ACE inhibitors (cough)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired accountant",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 61",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Ischemic stroke at age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Glyburide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "No prior major surgeries",
  "Cataract extraction, right eye (age 66)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia",
  "Mild intermittent asthma"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Acute decompensated heart failure", "Cervical spondylotic myelopathy", "Lisinopril adverse effect", "Recurrent benign prostatic hyperplasia", "Weber syndrome (Midbrain infarction)", "Acute ischemic stroke", "Cognitive impairment", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2019-08-13, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic conditions

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic conditions

HISTORY OF PRESENT ILLNESS:
He has been compliant with his medications, which include metformin and lisinopril. He reports no new symptoms and feels generally well. The patient's son, who accompanies him, mentions that his father has been more forgetful lately, but the patient attributes this to normal aging.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Glyburide 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

VITAL SIGNS:
His blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
His physical examination is unremarkable

LABORATORY DATA:
His laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 154 mg/dL
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 200 mg/dL

PLAN:
The physician decides to further investigate the patient's cognitive function.

## Encounter 1: source 2020-03-15, practice 2019-09-30, outpatient, Internal Medicine

Chief complaint: Dry, nonproductive cough for 4 weeks

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Dry, nonproductive cough for 4 weeks

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his chronic conditions. He presents for evaluation of a dry, nonproductive cough that began 4 weeks ago. He denies dyspnea, sputum production, fever, chills, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

His medical history includes hypertension, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
Medications are metformin, atorvastatin, albuterol as needed for mild intermittent asthma, and lisinopril started 6 weeks ago after a routine visit.

Home Medications:
- Glyburide 5 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
He does not smoke, drink alcohol, or use illicit drugs.

PHYSICAL EXAMINATION:
Physical examination shows normal breath sounds with a slightly prolonged expiratory phase but no wheezes, crackles, or rales. Cardiac examination is normal; there is no lower-extremity edema or jugular venous distension.

IMAGING:
Chest x-ray obtained earlier this week for an insurance physical is unremarkable.

## Encounter 2: source 2020-07-15, practice 2020-01-27, outpatient, Urology

Chief complaint: Worsening lower urinary tract symptoms after recent retention episode

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Wilson, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening lower urinary tract symptoms after recent retention episode

HISTORY OF PRESENT ILLNESS:
The patient returns for urologic evaluation with chronic hypertension, diabetes, cognitive impairment, and recent medication-related cough. He reports nocturia (3–4 times per night), weak urinary stream, urinary urgency, and difficulty initiating urination. He has also experienced two episodes of painless gross hematuria and was hospitalized last month for acute urinary retention requiring catheterization. Six months ago, he was started on tamsulosin, which helped initially, but symptoms have returned.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

His medical history includes hypertension, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Digital rectal exam reveals a firm, symmetrically enlarged prostate.

LABORATORY DATA:
Creatinine is normal.

IMAGING:
A recent bladder ultrasound shows increased prostate volume and a post-void residual volume of 220 mL.

## Encounter 3: source 2021-07-15, practice 2021-02-09, outpatient, Neurology

Chief complaint: Progressive difficulty walking and hand numbness over 6 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking and hand numbness over 6 months

HISTORY OF PRESENT ILLNESS:
The patient with cognitive impairment returns. He reports his legs feel stiff and he has tripped several times recently.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

His past medical history includes hypertension, type 2 diabetes mellitus, and hypercholesterolemia.

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On physical examination, you note decreased grip strength and atrophy of the intrinsic hand muscles. Reflexes are diminished in the upper extremities but hyperreflexic in the lower extremities. A positive Babinski sign is present bilaterally.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 7.2%
- Fasting glucose: 156 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

IMAGING:
MRI of the cervical spine reveals multilevel degenerative changes with evidence of spinal canal narrowing at the C5-C6 level.

## Encounter 4: source 2023-01-15, practice 2022-08-10, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and bilateral leg swelling over 3 months

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and bilateral leg swelling over 3 months

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with history of hypertension, diabetes, prior lisinopril-induced cough, and cervical myelopathy presents with a 3-month history of progressive dyspnea on exertion and bilateral lower extremity edema. He denies chest pain, palpitations, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Cervical spondylotic myelopathy (diagnosed 2021-07-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
His medications include metformin, amlodipine, and atorvastatin.

Home Medications:
- Glyburide 5 mg PO daily
- Lisinopril 20 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
He has jugular venous distension, crackles at the lung bases, and a 2+ pitting edema up to the shins. The cardiac examination reveals a laterally displaced point of maximal impulse and a third heart sound (S3).

LABORATORY DATA:
Laboratory findings are as follows:

- Serum sodium: 138 mEq/L
- Serum potassium: 4.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL

IMAGING:
- Chest X-ray: Enlarged cardiac silhouette with cephalization of pulmonary vasculature

## Encounter 5: source 2025-01-15, practice 2024-08-01, ed, Emergency Medicine

Chief complaint: Sudden severe chest pain radiating to left arm with nausea and diaphoresis

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe chest pain radiating to left arm with nausea and diaphoresis

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with hypertension, type 2 diabetes, hyperlipidemia, and prior heart failure who presents to the emergency department with a 2-hour history of sudden-onset, severe chest pain radiating to his left arm, associated with nausea and diaphoresis. He describes the pain as "pressure-like" and rates it 9 out of 10.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Cervical spondylotic myelopathy (diagnosed 2021-07-15)
- Acute decompensated heart failure (diagnosed 2023-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Glyburide 5 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

VITAL SIGNS:
On examination, his blood pressure is 160/90 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths per minute.

LABORATORY DATA:
His cardiac biomarkers are as follows:

- Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 50 U/L (normal 0-24 U/L)
- Myoglobin: 150 mcg/L (normal 28-72 mcg/L)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF.

## Encounter 6: source 2026-01-15, practice 2025-08-03, ed, Emergency Medicine

Chief complaint: Sudden left-sided weakness and difficulty speaking

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden left-sided weakness and difficulty speaking

HISTORY OF PRESENT ILLNESS:
This 74-year-old male with hypertension, type 2 diabetes mellitus, hyperlipidemia, and a history of myocardial infarction and heart failure is brought to the emergency department by his wife after he developed sudden weakness on the left side of his body and difficulty speaking while eating breakfast. His symptoms started approximately 2 hours ago, and he is given intravenous medication to improve neurologic recovery.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Cervical spondylotic myelopathy (diagnosed 2021-07-15)
- Acute decompensated heart failure (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2025-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

He has a history of hypertension, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
His medications include lisinopril, metformin, and atorvastatin.

Home Medications:
- Glyburide 5 mg PO daily
- Aspirin 81 mg PO daily
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

VITAL SIGNS:
On arrival, his blood pressure is 154/92 mmHg, pulse is 84/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Neurologic examination reveals left facial droop sparing the forehead, decreased strength in the left upper and lower extremities, and expressive aphasia.

IMAGING:
Non-contrast CT scan of the head shows no hemorrhage.

## Encounter 7: source 2027-01-15, practice 2026-08-02, ed, Emergency Medicine

Chief complaint: Sudden dizziness, nausea, and difficulty walking for 4 hours

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden dizziness, nausea, and difficulty walking for 4 hours

HISTORY OF PRESENT ILLNESS:
This 75-year-old male with hypertension, type 2 diabetes, hyperlipidemia, asthma, and an ischemic stroke in 2026 is brought to the emergency department by his wife for a 4-hour history of sudden-onset dizziness, nausea, and difficulty walking. He reports the room is spinning and has vomited twice since symptoms began.

PAST MEDICAL HISTORY:
Active Problem List:
- Cognitive impairment (diagnosed 2020-01-15)
- Lisinopril adverse effect (diagnosed 2020-03-15)
- Recurrent benign prostatic hyperplasia (diagnosed 2020-07-15)
- Cervical spondylotic myelopathy (diagnosed 2021-07-15)
- Acute decompensated heart failure (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2025-01-15)
- Acute ischemic stroke (diagnosed 2026-01-15)
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia
- Mild intermittent asthma

He has a 30-year history of hypertension and type II diabetes.

PAST SURGICAL HISTORY:
- No prior major surgeries
- Cataract extraction, right eye (age 66)

MEDICATIONS:
His medications are lisinopril, atorvastatin, aspirin, metformin, glyburide, and a daily multi-vitamin.

Home Medications:
- Albuterol HFA inhaler 90 mcg 2 puffs inhaled every 4-6 hours as needed for wheezing

ALLERGIES:
ACE inhibitors (cough)

FAMILY HISTORY:
- Father: Myocardial infarction at age 61
- Mother: Type 2 diabetes mellitus
- Brother: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On examination, he has horizontal nystagmus, worse with gaze to the right. There also appears to be a mild Horner syndrome on the right. There is right limb ataxia and dysmetria of the right upper and lower limbs on finger-to-nose and heel-to-shin testing. Muscle strength is preserved, and there are no signs of weakness or sensory loss. His speech is mildly slurred, and he veers to the right when attempting to walk.

ASSESSMENT AND PLAN:
A stroke at
