# synthetic_hospital patient 2248

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 69 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-04 (Tue) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2024-04-06 (Sat) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2024-09-30 (Mon) | procedure | Cardiology |
| 3 | 2021-12-15 | 2025-03-10 (Mon) | outpatient | Internal Medicine |
| 4 | 2022-01-15 | 2025-04-01 (Tue) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2025-10-06 (Mon) | ed | Emergency Medicine |
| 6 | 2022-12-15 | 2026-03-10 (Tue) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-04-07 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Hypertension, type 2 diabetes mellitus",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago (25 pack-year history)",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "30 mg daily (started 2 months ago)",
   "name": "Nifedipine extended-release"
  },
  {
   "dose": "500 mg every 6 hours as needed for joint pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with drug-eluting stent placement (3 years ago)",
  "Cardiac catheterization (6 months ago)",
  "Diagnostic cardiac catheterization (1 day ago)"
 ],
 "chronic_conditions": [
  "Coronary artery disease",
  "Hypertension",
  "Hyperlipidemia",
  "History of myocardial infarction (3 years prior)",
  "History of myocardial infarction (6 months prior)",
  "Chronic stable angina",
  "Osteoarthritis",
  "Major depressive disorder, in partial remission"
 ],
 "age_at_first_encounter": 69
}
```

Primary diagnoses: ["Upper Gastrointestinal Hemorrhage", "Anesthetic medication reaction", "Third-degree atrioventricular block", "Mobitz type I atrioventricular block (Wenckebach phenomenon)", "Hemodynamically unstable wide-complex tachycardia", "Retroperitoneal hematoma following cardiac catheterization", "Calcium channel blocker-induced peripheral edema", "Major depressive disorder"]
Comorbidities: ["Old myocardial infarction", "Stable Angina Pectoris (Medical Management)", "Essential hypertension", "Coronary Artery Disease (Antiplatelet Therapy)"]

## Encounter 0: source 2020-01-15, practice 2023-04-04, outpatient, Internal Medicine

Chief complaint: Concerns about his mood and behavior over the past month, brought by his daughter due to increasing withdrawal and loss of interest in hobbies.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Concerns about his mood and behavior over the past month, brought by his daughter due to increasing withdrawal and loss of interest in hobbies.

HISTORY OF PRESENT ILLNESS:
A 67-year-old man is brought to the primary care clinic by his daughter due to concerns about his mood and behavior over the past month. She says that he has become increasingly withdrawn and has lost interest in his usual hobbies, such as gardening and watching football. He spends most of the day lying in bed and requires prompting to eat or shower. He complains of waking up around 3 AM and being unable to fall back asleep. He denies suicidal thoughts but admits to feeling “tired of everything.” He has unintentionally lost 5 kg (11 lb) over the past 3 weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

His medical history is notable for hypertension and chronic stable angina.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
He takes for which he takes lisinopril and propranolol.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily at bedtime
- Metoprolol succinate 50 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On exam, he appears thin, avoids eye contact, and speaks slowly.

## Encounter 1: source 2021-01-15, practice 2024-04-06, ed, Emergency Medicine

Chief complaint: Collapsed while walking to the kitchen.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapsed while walking to the kitchen.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

He has a history of hypertension and coronary artery disease.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
Medications include aspirin, atorvastatin, and lisinopril

Home Medications:
- Metoprolol succinate 50 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
His pulse is 38/min and regular, blood pressure is 88/60 mm Hg, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
On arrival, he is alert but fatigued
Cardiopulmonary examination reveals distant heart sounds but is otherwise unremarkable

OTHER STUDIES:
ECG findings are shown below

## Encounter 2: source 2021-07-15, practice 2024-09-30, procedure, Cardiology

Chief complaint: During contrast administration for elective cardiac catheterization, he suddenly becomes flushed, hypotensive, and obtunded.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Park, MD
Department: Cardiology
Visit Type: procedure

CHIEF COMPLAINT:
During contrast administration for elective cardiac catheterization, he suddenly becomes flushed, hypotensive, and obtunded.

HISTORY OF PRESENT ILLNESS:
This patient with coronary artery disease and recent myocardial infarction is undergoing elective cardiac catheterization after being diagnosed with stable angina via a recent exercise stress test. Prior to the procedure, he was comfortable and alert.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-01-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

His medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
He does not take beta blockers.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily at bedtime
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
His temperature is 98.4°F (36.9°C), heart rate is 118/min, blood pressure is 78/40 mmHg, respiratory rate is 26/min, and oxygen saturation is 95% on room air.

PHYSICAL EXAMINATION:
On physical exam, he is tachycardic and warm to the touch with bounding peripheral pulses. There are no signs of jugular venous distension, rales, or peripheral edema. His skin is erythematous, and he has no urticaria or wheezing.

## Encounter 3: source 2021-12-15, practice 2025-03-10, outpatient, Internal Medicine

Chief complaint: Progressive swelling in his lower extremities over the past 3 weeks since starting nifedipine.

ENCOUNTER NOTE
Date: 2021-12-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling in his lower extremities over the past 3 weeks since starting nifedipine.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with coronary artery disease, prior myocardial infarctions, and recent cardiac catheterization complicated by contrast reaction who returns for evaluation of progressive lower extremity edema. He states that his ankles and feet feel heavy by the evening and that his shoes no longer fit properly by dinnertime. Each morning, the swelling is mostly resolved but worsens again throughout the day. He denies any shortness of breath, orthopnea, or paroxysmal nocturnal dyspnea. There is no chest pain, palpitations, or recent weight gain. He denies recent travel, changes in diet, or prolonged immobility.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-01-15)
- Anesthetic medication reaction (diagnosed 2021-07-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

His past medical history includes hypertension, stable angina, and osteoarthritis.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
He takes multiple medications including lisinopril, metoprolol, low-dose aspirin, simvastatin, acetaminophen as needed for joint pain, and nifedipine for blood pressure that was started 2 months ago. He has been on the same medication regimen for several months without any recent changes. He reports adherence to all prescribed medications.

Home Medications:
- Atorvastatin 40 mg daily at bedtime
- Metoprolol succinate 50 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
He is retired and enjoys walking his dog each morning and gardening in the afternoon. He sleeps well and has not noticed any cough, wheezing, or night sweats. He drinks alcohol socially and has never smoked.

REVIEW OF SYSTEMS:
On review of systems, he denies muscle pain, urinary symptoms, or skin changes. His appetite is normal and he has not experienced fatigue or dizziness.

VITAL SIGNS:
On physical exam, vital signs are within normal limits. His blood pressure today is 132/78 mm Hg.

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable, with no rales, wheezing, or murmurs. His jugular veins are not distended. There is 2+ bilateral pitting edema of the ankles that extends to the mid-shin. No erythema or warmth is noted. Calves are symmetric and non-tender. Abdominal exam is benign with no hepatosplenomegaly or ascites. There are no bruits, and bowel sounds are normal. Neurologic exam shows normal reflexes and strength.

LABORATORY DATA:
Laboratory studies including creatinine, liver function tests, and urinalysis are within normal limits.

IMAGING:
Chest x-ray is clear, with no signs of pulmonary congestion or cardiomegaly.

OTHER STUDIES:
An ECG shows normal sinus rhythm.

## Encounter 4: source 2022-01-15, practice 2025-04-01, ed, Emergency Medicine

Chief complaint: Collapsed while walking to the kitchen.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapsed while walking to the kitchen.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-01-15)
- Anesthetic medication reaction (diagnosed 2021-07-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2021-12-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

He has a history of hypertension and coronary artery disease.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
Medications include aspirin, atorvastatin, and lisinopril

Home Medications:
- Metoprolol succinate 50 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
His pulse is 42/min and regular, blood pressure is 88/60 mm Hg, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
On arrival, he is alert but fatigued
Cardiopulmonary examination reveals distant heart sounds but is otherwise unremarkable

OTHER STUDIES:
ECG findings are shown below

## Encounter 5: source 2022-07-15, practice 2025-10-06, ed, Emergency Medicine

Chief complaint: Collapsed while mowing his lawn with subsequent weakness and dizziness.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapsed while mowing his lawn with subsequent weakness and dizziness.

HISTORY OF PRESENT ILLNESS:
The patient with CAD and prior AV block presents. He had a brief episode of unconsciousness and awoke weak and dizzy.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-01-15)
- Anesthetic medication reaction (diagnosed 2021-07-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2021-12-15)
- Third-degree atrioventricular block (diagnosed 2022-01-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

He has a history of coronary artery disease and suffered a myocardial infarction 6 months ago.

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
Medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
In the ambulance, he was found to be hypotensive with a rapid pulse of 160/min.

PHYSICAL EXAMINATION:
On arrival, his mental status is altered and his blood pressure remains low despite IV fluids.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 6: source 2022-12-15, practice 2026-03-10, ed, Emergency Medicine

Chief complaint: Sudden onset of severe left flank pain and dizziness 2 hours ago following cardiac catheterization 1 day prior.

ENCOUNTER NOTE
Date: 2022-12-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe left flank pain and dizziness 2 hours ago following cardiac catheterization 1 day prior.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with coronary artery disease and prior myocardial infarction presents to the emergency department following cardiac catheterization performed 1 day prior due to unstable angina. He reports sudden onset of severe left flank pain and dizziness that started 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-01-15)
- Anesthetic medication reaction (diagnosed 2021-07-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2021-12-15)
- Third-degree atrioventricular block (diagnosed 2022-01-15)
- Hemodynamically unstable wide-complex tachycardia (diagnosed 2022-07-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
His current medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
Blood pressure is 90/50 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
On examination, he appears pale and diaphoretic. His abdomen is soft but there is tenderness to palpation in the left flank region without peritoneal signs.

LABORATORY DATA:
Hemoglobin has dropped from 14 g/dL pre-catheterization to 10 g/dL.

IMAGING:
A bedside ultrasound shows no evidence of intra-abdominal free fluid.

OTHER STUDIES:
His ECG shows no new changes.

## Encounter 7: source 2023-01-15, practice 2026-04-07, ed, Emergency Medicine

Chief complaint: Severe, crampy abdominal pain and multiple episodes of bright red blood per rectum over 2 hours.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, crampy abdominal pain and multiple episodes of bright red blood per rectum over 2 hours.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with coronary artery disease, prior myocardial infarctions, and recent retroperitoneal hematoma presents to the emergency department with a 2-hour history of severe, crampy abdominal pain and multiple episodes of bright red blood per rectum. He appears pale and diaphoretic.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Mobitz type I atrioventricular block (Wenckebach phenomenon) (diagnosed 2021-01-15)
- Anesthetic medication reaction (diagnosed 2021-07-15)
- Calcium channel blocker-induced peripheral edema (diagnosed 2021-12-15)
- Third-degree atrioventricular block (diagnosed 2022-01-15)
- Hemodynamically unstable wide-complex tachycardia (diagnosed 2022-07-15)
- Retroperitoneal hematoma following cardiac catheterization (diagnosed 2022-12-15)
- Coronary artery disease
- Hypertension
- Hyperlipidemia
- History of myocardial infarction (3 years prior)
- History of myocardial infarction (6 months prior)
- Chronic stable angina
- Osteoarthritis
- Major depressive disorder, in partial remission

PAST SURGICAL HISTORY:
- Cardiac catheterization with drug-eluting stent placement (3 years ago)
- Cardiac catheterization (6 months ago)
- Diagnostic cardiac catheterization (1 day ago)

MEDICATIONS:
His current medications include aspirin, atorvastatin, and metoprolol.

Home Medications:
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Nifedipine extended-release 30 mg daily (started 2 months ago)
- Acetaminophen 500 mg every 6 hours as needed for joint pain

ALLERGIES:
Iodinated contrast media - anaphylactoid reaction (flushing, hypotension, obtundation)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension, type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago (25 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired.

VITAL SIGNS:
His blood pressure is 95/60 mmHg, heart rate is 110 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a soft abdomen with mild diffuse tenderness but no rebound or guarding. Rectal examination confirms bright red blood.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 8.2 g/dL
- Hematocrit: 24.5%
- Platelets: 220 x 10^3/uL
- INR: 1.0
- Creatinine: 1.1 mg/dL
- BUN: 29 mg/dL
