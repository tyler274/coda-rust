# synthetic_hospital patient 2273

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-31 (Tue) | outpatient | Dermatology |
| 1 | 2020-05-15 | 2022-10-05 (Wed) | outpatient | Otolaryngology |
| 2 | 2021-03-15 | 2023-07-26 (Wed) | outpatient | Urology |
| 3 | 2022-03-15 | 2024-08-06 (Tue) | outpatient | Pulmonology |
| 4 | 2022-09-15 | 2025-01-28 (Tue) | outpatient | Cardiology |
| 5 | 2023-03-15 | 2025-08-04 (Mon) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-12-01 (Mon) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-05-27 (Wed) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction worker",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Hypertension",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg twice daily",
   "name": "Metoprolol"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg daily",
   "name": "Furosemide"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Congestive heart failure",
  "Hyperlipidemia",
  "Osteoarthritis",
  "History of pulmonary tuberculosis (resolved, treated 20 years ago)"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Carotid artery stenosis", "Bladder urothelial carcinoma (lateral wall)", "Acute decompensated heart failure", "Actinic keratosis", "Patient anger/acute situational irritability", "Statin-associated muscle symptoms", "Presbycusis", "Non-small cell lung cancer, right upper lobe"]
Comorbidities: ["Essential hypertension", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2022-05-31, outpatient, Dermatology

Chief complaint: Rough, scaly patch on left cheek slowly enlarging over the past year

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Rough, scaly patch on left cheek slowly enlarging over the past year

HISTORY OF PRESENT ILLNESS:
That has been slowly enlarging over the past year. He reports that the lesion occasionally itches but is mostly asymptomatic.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

His medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes for which he takes hydrochlorothiazide,.

Home Medications:
- Lisinopril 20 mg daily
- Metoprolol 50 mg twice daily
- Furosemide 40 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
He has a 30-pack-year smoking history but quit 10 years ago.
The patient is a retired construction worker and admits to having spent many years working outdoors without regular use of sunscreen.

PHYSICAL EXAMINATION:
On physical examination, you note a 1.5 cm erythematous, scaly plaque with a slightly raised, irregular border on the sun-exposed area of his left cheek. There are several similar but smaller lesions on his scalp and dorsal hands. He has no lymphadenopathy.

## Encounter 1: source 2020-05-15, practice 2022-10-05, outpatient, Otolaryngology

Chief complaint: Daughter reports patient has become withdrawn with difficulty hearing conversations

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. James Okafor, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Daughter reports patient has become withdrawn with difficulty hearing conversations

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with hypertension, congestive heart failure, hyperlipidemia, osteoarthritis, and history of resolved pulmonary tuberculosis presents for Otolaryngology evaluation of hearing concerns. His daughter has noticed that he often does not respond when she calls him from another room and seems to be watching television with the volume much higher than before. The patient admits to feeling frustrated with his difficulty in following conversations, especially in noisy environments, and has started to avoid social gatherings.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

His past medical history includes hypertension, for which he takes hydrochlorothiazide.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metoprolol 50 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Furosemide 40 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction worker.

REVIEW OF SYSTEMS:
He denies any otalgia, otorrhea, or vertigo.

PHYSICAL EXAMINATION:
On physical examination, the patient has a normal otoscopic exam, and the whispered voice test reveals a decreased ability to hear whispered phrases bilaterally. Tuning fork tests (Rinne and Weber) are unremarkable. No other abnormalities are noted.

## Encounter 2: source 2021-03-15, practice 2023-07-26, outpatient, Urology

Chief complaint: Two-month history of painless hematuria

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of painless hematuria

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, congestive heart failure, hyperlipidemia, osteoarthritis, and resolved pulmonary tuberculosis, previously treated for actinic keratosis and presbycusis, presents with two months of painless hematuria. He has no history of dysuria, urgency, or frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Presbycusis (diagnosed 2020-05-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

His past medical history is significant for hypertension, which is well-controlled.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes metoprolol.

Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Furosemide 40 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
40-pack-year smoking history, although he quit smoking 10 years ago

VITAL SIGNS:
Blood pressure is 135/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
On physical examination, the rest of the exam is unremarkable

LABORATORY DATA:
Urinalysis reveals red blood cells but no casts or infection. A complete blood count and serum chemistry panel are within normal limits.

IMAGING:
A subsequent CT urogram shows a 3 cm mass on the lateral wall of the bladder.

## Encounter 3: source 2022-03-15, practice 2024-08-06, outpatient, Pulmonology

Chief complaint: Persistent cough and unexplained weight loss over three months

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Thompson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and unexplained weight loss over three months

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with hypertension and a history of bladder urothelial carcinoma. He presents to the clinic with a persistent cough and unexplained weight loss over the past 3 months. He denies any fever, night sweats, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Presbycusis (diagnosed 2020-05-15)
- Bladder urothelial carcinoma (lateral wall) (diagnosed 2021-03-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and a remote history of tuberculosis treated 20 years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Metoprolol 50 mg twice daily
- Furosemide 40 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic. Lung auscultation reveals decreased breath sounds in the right upper lobe. No lymphadenopathy is palpable.

LABORATORY DATA:
Laboratory values are as follows:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Calcium: 10.2 mg/dL
- Alkaline phosphatase: 120 U/L
- Carcinoembryonic antigen (CEA): Elevated

IMAGING:
A chest X-ray shows a right upper lobe mass, and a CT scan confirms a 4 cm spiculated mass with no evidence of metastasis.

## Encounter 4: source 2022-09-15, practice 2025-01-28, outpatient, Cardiology

Chief complaint: Bilateral leg pain worsening over two months

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Aisha Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral leg pain worsening over two months

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with hypertension, heart failure, hyperlipidemia, and recent lung cancer who returns with bilateral leg pain that has been worsening over the past two months. He describes the pain as a constant, dull ache that intensifies with walking and subsides with rest. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Presbycusis (diagnosed 2020-05-15)
- Bladder urothelial carcinoma (lateral wall) (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-03-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes atorvastatin 40 mg daily and lisinopril 20 mg daily.

Home Medications:
- Metoprolol 50 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Furosemide 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction worker.

PHYSICAL EXAMINATION:
Physical examination reveals no skin changes, palpable pulses are intact, and there is no edema. Muscle strength is 5/5 in all extremities, but the patient reports tenderness in the proximal muscles of his lower limbs.

LABORATORY DATA:
- Creatine kinase (CK): 600 U/L (Normal: 38-174 U/L)
- LDL cholesterol: 90 mg/dL (Normal: <100 mg/dL for patients with high cardiovascular risk)
- AST: 40 U/L (Normal: 10-40 U/L)
- ALT: 45 U/L (Normal: 7-56 U/L)
- TSH: 2.5 mIU/L (Normal: 0.4-4.0 mIU/L)

## Encounter 5: source 2023-03-15, practice 2025-08-04, ed, Emergency Medicine

Chief complaint: Worsening dyspnea over past week with 5 kg weight gain and increased leg swelling

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening dyspnea over past week with 5 kg weight gain and increased leg swelling

HISTORY OF PRESENT ILLNESS:
The patient with congestive heart failure and recent cardiology evaluation presents with fluid overload. He reports a weight gain of 5 kg and has noted increased swelling in his legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Presbycusis (diagnosed 2020-05-15)
- Bladder urothelial carcinoma (lateral wall) (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-03-15)
- Statin-associated muscle symptoms (diagnosed 2022-09-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes furosemide, lisinopril, and aspirin.

Home Medications:
- Metoprolol 50 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction worker.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals decreased breath sounds at the bases bilaterally without crackles. Jugular venous pressure is elevated. Lower extremity pitting edema is present.

LABORATORY DATA:
Thoracentesis is performed, and pleural fluid analysis reveals the following:

- Pleural fluid protein: 1.5 g/dL
- Serum protein: 6.0 g/dL
- Pleural fluid LDH: 110 U/L
- Serum LDH: 200 U/L
- Pleural fluid-to-serum protein ratio: <0.5
- Pleural fluid-to-serum LDH ratio: <0.6

IMAGING:
A chest X-ray shows bilateral pleural effusions

## Encounter 6: source 2023-07-15, practice 2025-12-01, ed, Emergency Medicine

Chief complaint: Sudden painless loss of vision in right eye lasting 10 minutes, two hours ago

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden painless loss of vision in right eye lasting 10 minutes, two hours ago

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with congestive heart failure, hypertension, and hyperlipidemia who presented four months ago with acute decompensated heart failure, and now comes to the physician 2 hours after he experienced a 10-minute episode of sudden, painless loss of vision in his right eye. He currently has no symptoms but the episode was extremely concerning and worrisome for him.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Presbycusis (diagnosed 2020-05-15)
- Bladder urothelial carcinoma (lateral wall) (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-03-15)
- Statin-associated muscle symptoms (diagnosed 2022-09-15)
- Acute decompensated heart failure (diagnosed 2023-03-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Metoprolol 50 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Furosemide 40 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction worker.

PHYSICAL EXAMINATION:
Fundoscopic examination shows no abnormalities.

## Encounter 7: source 2024-01-15, practice 2026-05-27, outpatient, Internal Medicine

Chief complaint: Routine checkup

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Washington, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine checkup

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with congestive heart failure, non-small cell lung cancer, bladder urothelial carcinoma, and carotid artery stenosis, returning for routine follow-up. He is usually punctual and cooperative at visits. Today, however, he appears visibly frustrated and curt in his responses. As the medical assistant begins the intake, the patient mutters that the waiting room was "a disaster," and complains loudly that he had to wait for over 30 minutes. When the physician enters the room and greets him, the patient interrupts: "I'm sick of this! Every time I come here, I wait forever. I'm not some nobody you can just ignore!" He crosses his arms and avoids eye contact, clearly agitated.

PAST MEDICAL HISTORY:
Active Problem List:
- Actinic keratosis (diagnosed 2020-01-15)
- Presbycusis (diagnosed 2020-05-15)
- Bladder urothelial carcinoma (lateral wall) (diagnosed 2021-03-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2022-03-15)
- Statin-associated muscle symptoms (diagnosed 2022-09-15)
- Acute decompensated heart failure (diagnosed 2023-03-15)
- Carotid artery stenosis (diagnosed 2023-07-15)
- Hypertension
- Congestive heart failure
- Hyperlipidemia
- Osteoarthritis
- History of pulmonary tuberculosis (resolved, treated 20 years ago)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Metoprolol 50 mg twice daily
- Hydrochlorothiazide 25 mg daily
- Furosemide 40 mg daily
- Atorvastatin 40 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Mother: Type 2 diabetes mellitus
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 10 years ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired construction worker.
