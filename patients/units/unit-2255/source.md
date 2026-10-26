# synthetic_hospital patient 2255

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-30 (Mon) | outpatient | Neurology |
| 1 | 2020-04-15 | 2023-05-08 (Mon) | outpatient | Cardiology |
| 2 | 2021-01-15 | 2024-02-05 (Mon) | outpatient | Cardiology |
| 3 | 2021-07-15 | 2024-08-02 (Fri) | outpatient | Neurology |
| 4 | 2022-07-15 | 2025-07-21 (Mon) | outpatient | Cardiology |
| 5 | 2023-01-15 | 2026-01-28 (Wed) | icu | Emergency Medicine |
| 6 | 2023-01-15 | 2026-01-28 (Wed) | inpatient | Cardiology |
| 7 | 2023-07-15 | 2026-07-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired pharmaceutical sales representative",
 "alcohol_use": "Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol)",
 "family_history": [
  "Father: Myocardial infarction at age 59",
  "Mother: Hypertension and hyperlipidemia",
  "Brother: Essential tremor",
  "Paternal uncle: Stroke at age 64"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (30 pack-years, quit 8 years ago)",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
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
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Essential tremor",
  "Coronary artery disease",
  "History of myocardial infarction",
  "Ischemic cardiomyopathy"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Essential tremor", "Weber syndrome (Midbrain infarction)", "Recurrent pericardial effusion (palliative management)", "Myoclonus status epilepticus (post-hypoxic)", "Ischemic cardiomyopathy", "Stable Angina Pectoris due to Coronary Artery Disease", "Stable Angina Pectoris (Medical Management)"]
Comorbidities: ["Old myocardial infarction", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-01-30, outpatient, Neurology

Chief complaint: 6-month history of bilateral hand tremors

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of bilateral hand tremors

HISTORY OF PRESENT ILLNESS:
He reports that the tremors are most noticeable when he is holding objects, like a cup of coffee or writing with a pen, and they improve with alcohol consumption. He denies any tremor at rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Essential tremor
- Coronary artery disease
- History of myocardial infarction
- Ischemic cardiomyopathy

His past medical history is significant for hypertension, controlled with lisinopril, and hypercholesterolemia, managed with atorvastatin.

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

PHYSICAL EXAMINATION:
On examination, you observe a postural tremor of both hands without rigidity or bradykinesia. There are no signs of cerebellar dysfunction, and the rest of the neurological examination is unremarkable.

LABORATORY DATA:
Routine blood work, including thyroid function tests, is within normal limits.

## Encounter 1: source 2020-04-15, practice 2023-05-08, outpatient, Cardiology

Chief complaint: Routine follow-up for ischemic cardiomyopathy

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for ischemic cardiomyopathy

HISTORY OF PRESENT ILLNESS:
This 66-year-old with ischemic cardiomyopathy returns for follow-up following recent Neurology evaluation for bilateral hand tremors. He reports compliance with his medication regimen. He mentions occasional shortness of breath with exertion but denies chest pain, palpitations, or syncope. He has completed a 6-month course of guideline-directed medical therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction
- Ischemic cardiomyopathy

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
He takes aspirin, metoprolol, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, heart rate is 78 bpm,

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for a III/VI systolic murmur best heard at the apex.

IMAGING:
An echocardiogram reveals left ventricular systolic dysfunction with an ejection fraction of 28%.

## Encounter 2: source 2021-01-15, practice 2024-02-05, outpatient, Cardiology

Chief complaint: Exertional chest pain resolving with rest, requesting specific new medication

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Exertional chest pain resolving with rest, requesting specific new medication

HISTORY OF PRESENT ILLNESS:
The patient with ischemic cardiomyopathy last seen in April 2020 returns and mentions that he has been reading about a new drug, Drug X, which he insists on being prescribed instead of other treatments.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Ischemic cardiomyopathy (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
He is currently taking atorvastatin and lisinopril.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
The patient is a retired pharmaceutical sales representative

VITAL SIGNS:
On examination, his blood pressure is 140/85 mmHg, heart rate is 78 bpm,

PHYSICAL EXAMINATION:
He has no murmurs, rubs, or gallops.

LABORATORY DATA:
His lipid panel is within normal limits.

OTHER STUDIES:
An electrocardiogram shows no acute ST changes,

ASSESSMENT AND PLAN:
After further evaluation, you diagnose him with stable angina. You are aware that Drug X is a novel agent with limited long-term data and is not considered first-line therapy for stable angina.

## Encounter 3: source 2021-07-15, practice 2024-08-02, outpatient, Neurology

Chief complaint: Progressive bilateral hand tremors over 2 years

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral hand tremors over 2 years

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, hyperlipidemia, coronary artery disease with prior myocardial infarction, and ischemic cardiomyopathy returns to neurology clinic for follow-up of hand tremors initially evaluated January 2020. He presents with a 2-year history of progressive, bilateral hand tremors that worsen with movement, such as when drinking from a cup or writing. He states the tremors are less noticeable at rest and do not interfere with his sleep. He denies rigidity, bradykinesia, or other neurological symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Ischemic cardiomyopathy (diagnosed 2020-04-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

PHYSICAL EXAMINATION:
On physical examination, you observe a postural tremor in both hands without rigidity or bradykinesia. His handwriting shows consistent fluctuations with large, wavy letters. There is no evidence of decrementing micrographia. His gait is normal, and there are no signs of postural instability.

## Encounter 4: source 2022-07-15, practice 2025-07-21, outpatient, Cardiology

Chief complaint: Chest pain on exertion with recent syncopal episode

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Chest pain on exertion with recent syncopal episode

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent chest pain in the setting of CAD and prior stable angina. The pain is described as a pressure-like sensation in the central chest, radiating to the left arm, and is relieved by rest. He also reports one episode of syncope while climbing stairs last week.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Ischemic cardiomyopathy (diagnosed 2020-04-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

VITAL SIGNS:
His blood pressure is 140/85 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
A systolic ejection murmur is heard at the right upper sternal border, radiating to the carotids

IMAGING:
An echocardiogram reveals a calcified aortic valve with a valve area of 1.5 cm2 and normal left ventricular function.

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.

## Encounter 5: source 2023-01-15, practice 2026-01-28, icu, Emergency Medicine

Chief complaint: Found unresponsive after collapse, ventricular fibrillation arrest

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unresponsive after collapse, ventricular fibrillation arrest

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with coronary artery disease, ischemic cardiomyopathy, and essential tremor, last evaluated for exertional chest pain and syncope. He presents acutely after collapse. His wife reports that he collapsed suddenly while walking to the kitchen. She immediately called emergency services and initiated cardiopulmonary resuscitation. On arrival, paramedics found the patient in ventricular fibrillation and delivered three shocks with an automated external defibrillator before return of spontaneous circulation was achieved. The patient remains comatose with a Glasgow Coma Scale score of 6.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Ischemic cardiomyopathy (diagnosed 2020-04-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction

His past medical history includes hypertension, hyperlipidemia, and a previous myocardial infarction.

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

VITAL SIGNS:
His temperature is 37.2°C, blood pressure is 110/70 mmHg, pulse is 88 beats per minute, and respiratory rate is 16 beats per minute with mechanical ventilation.

PHYSICAL EXAMINATION:
On examination, the patient exhibits continuous, generalized, symmetric myoclonic jerks involving his axial, limb, and facial muscles.

LABORATORY DATA:
Laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.1 mg/dL
- Glucose: 98 mg/dL
- Serum calcium: 9.2 mg/dL
- Magnesium: 2.0 mEq/L
- Phosphate: 3.5 mg/dL
- Complete blood count: within normal limits
- Arterial blood gas: pH 7.40, PaCO2 40 mmHg, PaO2 95 mmHg

OTHER STUDIES:
An EEG shows no epileptiform discharges.

## Encounter 6: source 2023-01-15, practice 2026-01-28, inpatient, Cardiology

Chief complaint: Sharp pleuritic chest pain worsening when supine, 4 days post-MI

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Sharp pleuritic chest pain worsening when supine, 4 days post-MI

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with known ischemic cardiomyopathy who returns four days following myocardial infarction complicated by ventricular fibrillation arrest. He presents to the emergency department with sharp, left-sided chest pain that worsens with deep breaths and when lying flat. He reports that the pain started earlier this morning and describes it as different from the pain he experienced four days ago when he was admitted for a myocardial infarction.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Ischemic cardiomyopathy (diagnosed 2020-04-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2022-07-15)
- Myoclonus status epilepticus (post-hypoxic) (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
His medications include aspirin, metoprolol, atorvastatin, and lisinopril.

Home Medications:
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

VITAL SIGNS:
On physical examination, his blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
Auscultation of the heart reveals a pericardial friction rub. Lung fields are clear to auscultation.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 11,000 /µL
- Hemoglobin: 14.2 g/dL
- Platelet count: 250,000 /µL
- Creatinine: 1.0 mg/dL
- Troponin I: 0.02 ng/mL (normal <0.04 ng/mL)

IMAGING:
An echocardiogram is performed to evaluate the patient's symptoms.

OTHER STUDIES:
An ECG shows sinus rhythm with no new changes since his previous ECG.

## Encounter 7: source 2023-07-15, practice 2026-07-21, ed, Emergency Medicine

Chief complaint: Acute onset left ptosis, down-and-out left eye, right-sided weakness

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset left ptosis, down-and-out left eye, right-sided weakness

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with ischemic cardiomyopathy, essential tremor, and cardiac arrest (January 2023) presents with neurological symptoms. He reports weakness in right arm and leg, concurrent with his eye symptoms. He denies any headache, dizziness, or loss of consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Ischemic cardiomyopathy (diagnosed 2020-04-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2022-07-15)
- Myoclonus status epilepticus (post-hypoxic) (diagnosed 2023-01-15)
- Recurrent pericardial effusion (palliative management) (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- History of myocardial infarction

PAST SURGICAL HISTORY:
- Cardiac catheterization with percutaneous coronary intervention (drug-eluting stent placement)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 59
- Mother: Hypertension and hyperlipidemia
- Brother: Essential tremor
- Paternal uncle: Stroke at age 64

SOCIAL HISTORY:
Smoking: Former smoker (30 pack-years, quit 8 years ago). Alcohol: Social (2-3 glasses of wine weekly; notes tremor improvement with alcohol). Occupation: Retired pharmaceutical sales representative.

VITAL SIGNS:
On physical examination, his blood pressure is 165/95 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Neurological examination reveals left-sided ptosis, a dilated left pupil that is non-reactive to light, and a left eye that is deviated downward and laterally. Motor examination shows 3/5 strength in the right upper and lower extremities with normal tone and reflexes. Sensory examination is intact.

IMAGING:
A CT scan of the head without contrast shows no hemorrhage or mass effect.
