# synthetic_hospital patient 2260

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-07 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2021-05-10 (Mon) | outpatient | Cardiology |
| 2 | 2021-05-15 | 2022-05-16 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2023-07-03 (Mon) | outpatient | Neurology |
| 4 | 2023-05-15 | 2024-05-01 (Wed) | ed | Emergency Medicine |
| 5 | 2023-11-15 | 2024-10-29 (Tue) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-07-11 (Fri) | icu | Cardiology |
| 7 | 2025-01-15 | 2025-12-29 (Mon) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired Accountant",
 "alcohol_use": "None",
 "family_history": [
  "Prostate cancer (father)",
  "Myocardial infarction (father)",
  "Stroke (mother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago, 30 pack-year history",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.4 mg nightly",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Benign Prostatic Hyperplasia"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Ventricular tachycardia with syncope", "Depressive disorder due to hypercalcemia", "Frontotemporal dementia", "Loss of consciousness (syncope)", "Prostate cancer screening indication", "Carotid artery stenosis", "Transient ischemic attack (amaurosis fugax) - antiplatelet therapy approach", "Right Ventricular Myocardial Infarction"]
Comorbidities: ["Recurrent benign prostatic hyperplasia", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-01-07, outpatient, Internal Medicine

Chief complaint: Annual wellness examination with discussion regarding ongoing PSA screening appropriateness given age and family history of prostate cancer.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Annual wellness examination with discussion regarding ongoing PSA screening appropriateness given age and family history of prostate cancer.

HISTORY OF PRESENT ILLNESS:
He has no complaints and feels well. The patient inquires about the utility of continuing annual PSA screening given his age and family history.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

His past medical history is significant for hypertension, controlled with lisinopril, and hypercholesterolemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Lisinopril, atorvastatin

Home Medications:
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for prostate cancer in his father.

SOCIAL HISTORY:
He quit smoking 10 years ago after a 30-year pack history.

VITAL SIGNS:
On physical examination, his blood pressure is 132/84 mmHg, heart rate is 78 beats per minute, and BMI is 27 kg/m^2.

PHYSICAL EXAMINATION:
A digital rectal examination reveals a smooth, enlarged prostate without nodules.

LABORATORY DATA:
His prostate-specific antigen (PSA) levels have been monitored annually, with the most recent value being 4.2 ng/mL, slightly increased from 3.8 ng/mL the previous year.

## Encounter 1: source 2020-05-15, practice 2021-05-10, outpatient, Cardiology

Chief complaint: Routine cardiovascular risk assessment and asymptomatic carotid artery screening.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine cardiovascular risk assessment and asymptomatic carotid artery screening.

HISTORY OF PRESENT ILLNESS:
The patient presents to the cardiology clinic for routine cardiovascular risk assessment, following his Internal Medicine wellness visit four months prior where prostate cancer screening was discussed. He reports no symptoms but is concerned about his risk of stroke.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His medications include lisinopril and atorvastatin

Home Medications:
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
His brother recently had a transient ischemic attack

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 30 pack-year history. Alcohol: None. Occupation: Retired Accountant.

VITAL SIGNS:
Blood pressure is 132/84 mmHg

PHYSICAL EXAMINATION:
Physical examination is unremarkable

IMAGING:
Carotid duplex ultrasonography shows 70% stenosis of the right internal carotid artery

## Encounter 2: source 2021-05-15, practice 2022-05-16, outpatient, Internal Medicine

Chief complaint: Six-week history of worsening fatigue, poor concentration, low mood, morning headaches, and non-restorative sleep despite adequate time in bed.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Six-week history of worsening fatigue, poor concentration, low mood, morning headaches, and non-restorative sleep despite adequate time in bed.

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with hypertension, hyperlipidemia, and benign prostatic hyperplasia who previously underwent evaluation for prostate cancer screening and carotid artery stenosis. He returns to the Internal Medicine clinic presenting with a six-week history of worsening fatigue, poor concentration, and low mood. He reports waking most mornings with a dull headache and feeling unrested despite spending 8–9 hours in bed. His wife reports that he snores loudly, often stops breathing during sleep, and seems excessively sleepy during the day. He notes a lack of motivation and no longer enjoys watching sports or playing with his grandchildren. Over the past two months, he has unintentionally gained 7 kg (15 lb) and finds himself falling asleep during meetings at work. He denies suicidal ideation but admits to feeling 'slowed down' and unproductive. Further questioning reveals that he has been taking short naps throughout the day, including while watching television and sometimes while driving, which has caused concern for his safety. His wife confirms that he frequently wakes up gasping for air or choking, especially in the second half of the night. He reports feeling increasingly irritable and detached, stating that his memory has worsened and that he often forgets recent conversations or misplaces items. He has also experienced episodes of morning confusion and difficulty initiating tasks at work. He denies feelings of worthlessness or hopelessness but does feel frustrated about his declining energy levels and productivity.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-05-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

His past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
There is no family history of mood disorders, dementia, or sleep disorders.

SOCIAL HISTORY:
He does not drink alcohol or use drugs. He reports occasional caffeine intake but no sedatives, antihistamines, or recreational substances. His diet has worsened recently, consisting mostly of convenience foods and snacks, and he avoids exercise due to fatigue.

REVIEW OF SYSTEMS:
On review of systems, he denies chest pain, palpitations, orthopnea, or leg swelling. He has not noticed nocturia or early morning awakenings apart from the gasping episodes.

VITAL SIGNS:
On physical exam, he has a BMI of 32 kg/m² and blood pressure of 138/84 mm Hg.

PHYSICAL EXAMINATION:
His last physical exam was over a year ago. His throat exam reveals crowded oropharyngeal architecture and a thick neck circumference. Mental status exam shows a blunted affect, but his thought process is coherent and logical. He is oriented and without psychosis.

LABORATORY DATA:
His serum TSH and vitamin B12 levels are within normal limits.

OTHER STUDIES:
Overnight polysomnography reveals an apnea-hypopnea index (AHI) of 25, indicating moderate obstructive sleep apnea (OSA).

## Encounter 3: source 2022-07-15, practice 2023-07-03, outpatient, Neurology

Chief complaint: Progressive behavioral changes over the past year including social disinhibition, inappropriate comments, declining hygiene, and compulsive hoarding behaviors.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive behavioral changes over the past year including social disinhibition, inappropriate comments, declining hygiene, and compulsive hoarding behaviors.

HISTORY OF PRESENT ILLNESS:
A 68-year-old male with hypertension, hyperlipidemia, and benign prostatic hyperplasia returns for evaluation of progressive behavioral symptoms following a 2021 presentation for depressive symptoms and hypercalcemia. His partner reports that he has become increasingly inappropriate in social situations, making lewd comments to strangers and showing a lack of concern for personal hygiene. He has also developed a habit of collecting and hoarding napkins from restaurants. Despite these changes, he remains oriented to time, place, and person.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-05-15)
- Depressive disorder due to hypercalcemia (diagnosed 2021-05-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

His past medical history is significant for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Prostate cancer (father)
- Myocardial infarction (father)
- Stroke (mother)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 30 pack-year history. Alcohol: None. Occupation: Retired Accountant.

PHYSICAL EXAMINATION:
On physical examination, he is disheveled and attempts to eat non-food items in the examination room. His neurological examination is notable for impaired judgment and difficulty with problem-solving tasks, but his memory appears intact. His Mini-Mental State Examination score is 24/30, with deficits in attention and executive function.

LABORATORY DATA:
Routine blood work, including complete blood count, electrolytes, renal function, liver enzymes, and thyroid function tests, is within normal limits.

IMAGING:
A head CT scan shows atrophy of the frontal and anterior temporal lobes.

## Encounter 4: source 2023-05-15, practice 2024-05-01, ed, Emergency Medicine

Chief complaint: Recurrent episodes of transient loss of consciousness occurring during nocturnal urination, witnessed by spouse with observed pallor and brief convulsive movements.

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of transient loss of consciousness occurring during nocturnal urination, witnessed by spouse with observed pallor and brief convulsive movements.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with hypertension, hyperlipidemia, benign prostatic hyperplasia, carotid artery stenosis, and frontotemporal dementia presents with recurrent episodes of witnessed transient loss of consciousness. He reports that these episodes often occur while he is urinating, especially at night. His wife has witnessed these events and notes that he becomes pale and has a brief period of convulsions during the episode.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-05-15)
- Depressive disorder due to hypercalcemia (diagnosed 2021-05-15)
- Frontotemporal dementia (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

His past medical history includes benign prostatic hyperplasia, hypertension, and hypercholesterolemia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medications include tamsulosin, lisinopril, and atorvastatin.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Prostate cancer (father)
- Myocardial infarction (father)
- Stroke (mother)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 30 pack-year history. Alcohol: None. Occupation: Retired Accountant.

VITAL SIGNS:
On examination, his blood pressure is 145/90 mmHg, heart rate is 78 bpm, and he is afebrile.

PHYSICAL EXAMINATION:
His cardiovascular examination is unremarkable, and neurological examination shows no focal deficits.

LABORATORY DATA:
His laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.1 mg/dL
- Glucose: 98 mg/dL
- Hemoglobin: 14.2 g/dL

OTHER STUDIES:
An ECG shows normal sinus rhythm with no significant ST-T changes.

## Encounter 5: source 2023-11-15, practice 2024-10-29, ed, Emergency Medicine

Chief complaint: Sudden, painless monocular vision loss described as a descending shade over the right eye, lasting several minutes with spontaneous resolution.

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden, painless monocular vision loss described as a descending shade over the right eye, lasting several minutes with spontaneous resolution.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with carotid artery stenosis presents with visual symptoms. He describes it as a "shade coming down over my eye," which resolved spontaneously after a few minutes. He denies any headache, jaw claudication, or recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-05-15)
- Depressive disorder due to hypercalcemia (diagnosed 2021-05-15)
- Frontotemporal dementia (diagnosed 2022-07-15)
- Loss of consciousness (syncope) (diagnosed 2023-05-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Prostate cancer (father)
- Myocardial infarction (father)
- Stroke (mother)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 30 pack-year history. Alcohol: None. Occupation: Retired Accountant.

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
On physical examination, carotid bruits are auscultated bilaterally. Fundoscopic examination is unremarkable. His remaining physical examination is normal.

## Encounter 6: source 2024-07-15, practice 2025-07-11, icu, Cardiology

Chief complaint: Acute onset of severe, central, pressure-like chest pain radiating to the left arm beginning one hour prior to arrival.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of severe, central, pressure-like chest pain radiating to the left arm beginning one hour prior to arrival.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with hypertension, hyperlipidemia, carotid artery stenosis, frontotemporal dementia, and a history of transient ischemic attack and syncope who presents with chest pain beginning one hour prior to arrival. He describes the pain as severe, central, and pressure-like, radiating to his left arm.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-05-15)
- Depressive disorder due to hypercalcemia (diagnosed 2021-05-15)
- Frontotemporal dementia (diagnosed 2022-07-15)
- Loss of consciousness (syncope) (diagnosed 2023-05-15)
- Transient ischemic attack (amaurosis fugax) - antiplatelet therapy approach (diagnosed 2023-11-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

His medical history includes hypertension and hyperlipidemia, but he has not seen a physician in several years.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Atorvastatin 20 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Prostate cancer (father)
- Myocardial infarction (father)
- Stroke (mother)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 30 pack-year history. Alcohol: None. Occupation: Retired Accountant.

VITAL SIGNS:
On examination, his temperature is 37.0°C (98.6°F), blood pressure is 95/60 mm Hg, pulse is 75/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
He appears diaphoretic and anxious.
Cardiac examination reveals normal S1 and S2, without murmurs, rubs, or gallops. Lungs are clear to auscultation.

OTHER STUDIES:
ECG findings are shown below.

## Encounter 7: source 2025-01-15, practice 2025-12-29, icu, Cardiology

Chief complaint: Sudden loss of consciousness while walking without prodromal symptoms, witnessed by spouse without convulsions, with spontaneous recovery within one minute.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Lisa Wong, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Sudden loss of consciousness while walking without prodromal symptoms, witnessed by spouse without convulsions, with spontaneous recovery within one minute.

HISTORY OF PRESENT ILLNESS:
The patient presents with a witnessed syncopal episode six months following his right ventricular myocardial infarction, occurring in the context of prior cerebrovascular events and a previous nocturnal syncopal episode. He reports no preceding symptoms such as nausea, diaphoresis, or a sense of warmth. His wife, who witnessed the event, states that he did not exhibit any convulsive movements, and he regained consciousness within a minute. On further questioning, the patient mentions occasional palpitations over the past few months but has not sought medical attention for them.

PAST MEDICAL HISTORY:
Active Problem List:
- Prostate cancer screening indication (diagnosed 2020-01-15)
- Carotid artery stenosis (diagnosed 2020-05-15)
- Depressive disorder due to hypercalcemia (diagnosed 2021-05-15)
- Frontotemporal dementia (diagnosed 2022-07-15)
- Loss of consciousness (syncope) (diagnosed 2023-05-15)
- Transient ischemic attack (amaurosis fugax) - antiplatelet therapy approach (diagnosed 2023-11-15)
- Right Ventricular Myocardial Infarction (diagnosed 2024-07-15)
- Hypertension
- Hyperlipidemia
- Benign Prostatic Hyperplasia

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Prostate cancer (father)
- Myocardial infarction (father)
- Stroke (mother)

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago, 30 pack-year history. Alcohol: None. Occupation: Retired Accountant.

VITAL SIGNS:
Blood pressure of 135/85 mmHg, a regular pulse of 78 beats per minute

PHYSICAL EXAMINATION:
No carotid bruits. Cardiac examination shows no murmurs, rubs, or gallops.

LABORATORY DATA:
His laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Hemoglobin: 14.5 g/dL
- Troponin I: <0.03 ng/mL (normal)

OTHER STUDIES:
An ECG performed in the ED shows a brief run of ventricular tachycardia.
