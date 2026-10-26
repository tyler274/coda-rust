# synthetic_hospital patient 2243

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare Part B with supplemental Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-11 (Mon) | outpatient | Urology |
| 1 | 2020-07-15 | 2021-07-04 (Sun) | ed | Emergency Medicine |
| 2 | 2021-04-15 | 2022-04-04 (Mon) | outpatient | Neurology |
| 3 | 2022-01-15 | 2023-01-09 (Mon) | outpatient | Neurology |
| 4 | 2023-01-15 | 2024-01-08 (Mon) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2024-06-27 (Thu) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-07-06 (Sun) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-01-10 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare Part B with supplemental Blue Cross Blue Shield",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Former occasional use (1-2 drinks per month), quit 6 months ago",
 "family_history": [
  "Father: Myocardial infarction at age 72",
  "Mother: Ischemic stroke at age 78",
  "One older brother with hypertension",
  "No family history of Parkinson disease or psychiatric illness"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25-100 mg three times daily",
   "name": "Carbidopa-Levodopa"
  },
  {
   "dose": "1 mg every morning",
   "name": "Rasagiline"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "0.4 mg nightly at bedtime",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "Colonoscopy (5 years prior, normal)",
  "Cardiac catheterization with drug-eluting stent placement (LAD)",
  "Cardiac catheterization with drug-eluting stent placement (RCA)",
  "Cystoscopy (diagnostic)"
 ],
 "chronic_conditions": [
  "Parkinson disease (5-year duration)",
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease (status post NSTEMI and inferior wall STEMI)",
  "Benign prostatic hyperplasia",
  "Diverticulosis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Medication-induced psychosis (dopaminergic therapy)", "Parkinson disease with psychosis", "Central retinal artery occlusion", "Non-ST elevation myocardial infarction (NSTEMI)", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "Diverticulosis", "Recurrent benign prostatic hyperplasia", "Upper Gastrointestinal Hemorrhage"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Mild cognitive impairment"]

## Encounter 0: source 2020-01-15, practice 2021-01-11, outpatient, Urology

Chief complaint: Gradually weakening urinary stream with nocturia and sensation of incomplete bladder emptying.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually weakening urinary stream with nocturia and sensation of incomplete bladder emptying.

HISTORY OF PRESENT ILLNESS:
He reports a gradually weakening stream, occasional nocturnal urination, and a feeling that he doesn't fully empty his bladder. He denies pain with urination, blood in the urine, or any recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia
- Diverticulosis

His medical history includes well-controlled hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Rasagiline 1 mg every morning
- Metoprolol succinate 50 mg daily
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
He does not use tobacco or alcohol.

PHYSICAL EXAMINATION:
Physical examination reveals normal vital signs and a soft, non-distended abdomen. A digital rectal exam shows a symmetrically enlarged, smooth, non-tender prostate.

LABORATORY DATA:
Urinalysis is normal, and serum creatinine is stable. PSA is mildly elevated but age-appropriate.

PLAN:
The patient is started on a medication that addresses the underlying hormonal pathophysiology of his condition and can reduce prostate volume over time.

## Encounter 1: source 2020-07-15, practice 2021-07-04, ed, Emergency Medicine

Chief complaint: Sudden onset of painless bright red blood per rectum with small clots this morning.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of painless bright red blood per rectum with small clots this morning.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with diverticulosis and benign prostatic hyperplasia (last evaluated by Urology six months ago) presents with acute painless hematochezia. He reports waking feeling well, having breakfast, and then experiencing an urgent need to defecate, producing a large volume of red blood mixed with small clots. He denies abdominal pain, dizziness, fever, or recent changes in bowel habits. This has never happened before.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia
- Diverticulosis

The patient has a history of hypertension and hyperlipidemia, as well as occasional constipation.

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Rasagiline 1 mg every morning
- Metoprolol succinate 50 mg daily
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
He does not smoke, drinks alcohol occasionally, and lives independently with his spouse.

VITAL SIGNS:
Temperature is 36.9°C (98.4°F), blood pressure is 124/76 mm Hg, pulse is 88/min, and respirations are 14/min.

PHYSICAL EXAMINATION:
On arrival, he appears alert and in no acute distress. Cardiac and pulmonary examinations are normal. Abdominal examination reveals a soft, nondistended abdomen without tenderness, guarding, or palpable masses. Digital rectal examination shows bright red blood but no palpable masses or fissures.

LABORATORY DATA:
Initial laboratory studies show hemoglobin 11.2 g/dL, mean corpuscular volume 90 fL, platelets 230,000/mm³, and normal coagulation studies. Basic metabolic panel is normal.

PLAN:
Intravenous fluids are started, and the patient remains hemodynamically stable. Colonoscopy is arranged and performed within 12 hours. Endoscopic epinephrine injection and cauterization are performed with successful hemostasis.

## Encounter 2: source 2021-04-15, practice 2022-04-04, outpatient, Neurology

Chief complaint: Visual hallucinations of small animals running across the room for the past two months.

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Jennifer Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Visual hallucinations of small animals running across the room for the past two months.

HISTORY OF PRESENT ILLNESS:
The patient with five-year Parkinson disease and history of benign prostatic hyperplasia and diverticulosis returns for evaluation. He describes seeing small animals running across the room, which are not present upon inspection. He denies any recent changes in his medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Diverticulosis (diagnosed 2020-07-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia

His medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
He takes carbidopa/levodopa and rasagiline.

Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Lisinopril 20 mg daily
- Metoprolol succinate 50 mg daily
- Atorvastatin 40 mg nightly
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use (1-2 drinks per month), quit 6 months ago. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient has a resting tremor, bradykinesia, and cogwheel rigidity. His Mini-Mental State Examination score is 26/30, indicating mild cognitive impairment.

## Encounter 3: source 2022-01-15, practice 2023-01-09, outpatient, Neurology

Chief complaint: Visual hallucinations and paranoid delusions that started two months ago after beginning a new medication.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Jennifer Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Visual hallucinations and paranoid delusions that started two months ago after beginning a new medication.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with Parkinson disease who returns to the Neurology clinic approximately nine months after a prior evaluation for similar visual hallucinations. He presents with his wife, who reports that he has been experiencing visual hallucinations and paranoid delusions for the past two months. The patient is currently on carbidopa-levodopa and started on a dopamine agonist three months ago. His hallucinations involve seeing small animals in the house, which his wife confirms are not present. The patient has become increasingly suspicious of his neighbors, accusing them of stealing from him, although there is no evidence to support this.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Diverticulosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-04-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia

He has no past psychiatric history.

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
He takes carbidopa-levodopa, dopamine agonist, atorvastatin, metoprolol.

Home Medications:
- Rasagiline 1 mg every morning
- Lisinopril 20 mg daily
- Metoprolol succinate 50 mg daily
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use (1-2 drinks per month), quit 6 months ago. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his blood pressure is 130/80 mmHg, heart rate is 68 bpm, and he is afebrile.

PHYSICAL EXAMINATION:
Neurological examination shows a resting tremor, bradykinesia, and rigidity. There are no signs of infection or metabolic derangement. Mini-Mental State Examination score is 26/30, with deductions in the areas of attention and calculation.

## Encounter 4: source 2023-01-15, practice 2024-01-08, ed, Emergency Medicine

Chief complaint: Pressure-like chest pain radiating to left arm with shortness of breath, onset two hours ago while exerting.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Smith, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Pressure-like chest pain radiating to left arm with shortness of breath, onset two hours ago while exerting.

HISTORY OF PRESENT ILLNESS:
The 70-year-old male with history of coronary artery disease and Parkinson disease presents with chest pain that started 2 hours ago while he was mowing the lawn. The pain is described as a pressure-like sensation in the center of his chest, radiating to his left arm, and associated with shortness of breath. He denies any nausea, vomiting, or diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Diverticulosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-04-15)
- Medication-induced psychosis (dopaminergic therapy) (diagnosed 2022-01-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
His current medications include lisinopril and atorvastatin.

Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Rasagiline 1 mg every morning
- Metoprolol succinate 50 mg daily
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use (1-2 drinks per month), quit 6 months ago. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is 85 beats per minute, and respiratory rate is 18 breaths per minute. His oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild pallor.

LABORATORY DATA:
Cardiac biomarkers are as follows:

- Troponin I: 0.08 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 10 U/L (normal 0-5 U/L)
- Myoglobin: 90 ng/mL (normal 28-72 ng/mL)

OTHER STUDIES:
An electrocardiogram (ECG) shows no ST-segment elevations but does show T-wave inversions in leads V4-V6.

## Encounter 5: source 2023-07-15, practice 2024-06-27, ed, Emergency Medicine

Chief complaint: Severe substernal chest pain radiating to right arm with nausea and diaphoresis, onset two hours ago at rest.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe substernal chest pain radiating to right arm with nausea and diaphoresis, onset two hours ago at rest.

HISTORY OF PRESENT ILLNESS:
A 70-year-old male with coronary artery disease and Parkinson disease presents six months after his prior infarction with chest pain that started 2 hours ago while resting. The pain is described as a heavy sensation located in the substernal area, radiating to the right arm, with nausea and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Diverticulosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-04-15)
- Medication-induced psychosis (dopaminergic therapy) (diagnosed 2022-01-15)
- Non-ST elevation myocardial infarction (NSTEMI) (diagnosed 2023-01-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Rasagiline 1 mg every morning
- Lisinopril 20 mg daily
- Metoprolol succinate 50 mg daily
- Atorvastatin 40 mg nightly
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use (1-2 drinks per month), quit 6 months ago. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 97% on room air

PHYSICAL EXAMINATION:
On examination, the patient appears in distress. Physical examination reveals jugular vein distension, normal lung auscultation, and no peripheral edema.

LABORATORY DATA:
Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 50 U/L (normal 0-24 U/L)
- B-type natriuretic peptide (BNP): 80 pg/mL (normal 0-100 pg/mL)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF.

## Encounter 6: source 2024-07-15, practice 2025-07-06, ed, Emergency Medicine

Chief complaint: Two-hour history of painless rectal bleeding with passage of large blood clots.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-hour history of painless rectal bleeding with passage of large blood clots.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with diverticulosis on aspirin presents to the emergency department with recurrent rectal bleeding after a similar painless episode in 2020. He reports a 2-hour history of painless rectal bleeding, passing several large clots of bright red blood without any associated abdominal pain. He denies any recent trauma or use of nonsteroidal anti-inflammatory drugs.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Diverticulosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-04-15)
- Medication-induced psychosis (dopaminergic therapy) (diagnosed 2022-01-15)
- Non-ST elevation myocardial infarction (NSTEMI) (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2023-07-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia

His past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Rasagiline 1 mg every morning
- Lisinopril 20 mg daily
- Metoprolol succinate 50 mg daily
- Atorvastatin 40 mg nightly
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use (1-2 drinks per month), quit 6 months ago. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure is 100/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
On examination, he appears pale. Digital rectal examination confirms the presence of bright red blood.

LABORATORY DATA:
His laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (baseline 14 g/dL)
- Hematocrit: 30.5%
- Platelet count: 250,000/mm^3
- Prothrombin time (PT): 12 seconds
- International normalized ratio (INR): 1.0
- Blood urea nitrogen (BUN): 18 mg/dL
- Creatinine: 1.0 mg/dL

## Encounter 7: source 2025-01-15, practice 2026-01-10, ed, Emergency Medicine

Chief complaint: Sudden painless loss of vision in left eye described as a curtain coming down, onset two hours ago.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden painless loss of vision in left eye described as a curtain coming down, onset two hours ago.

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with Parkinson disease and coronary artery disease status post NSTEMI and inferior wall STEMI presents to the emergency department with a sudden loss of vision in his left eye that occurred 2 hours ago. He describes it as a "curtain coming down" over his eye. He denies any pain, headache, or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Diverticulosis (diagnosed 2020-07-15)
- Parkinson disease with psychosis (diagnosed 2021-04-15)
- Medication-induced psychosis (dopaminergic therapy) (diagnosed 2022-01-15)
- Non-ST elevation myocardial infarction (NSTEMI) (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2023-07-15)
- Upper Gastrointestinal Hemorrhage (diagnosed 2024-07-15)
- Parkinson disease (5-year duration)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (status post NSTEMI and inferior wall STEMI)
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Colonoscopy (5 years prior, normal)
- Cardiac catheterization with drug-eluting stent placement (LAD)
- Cardiac catheterization with drug-eluting stent placement (RCA)
- Cystoscopy (diagnostic)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Carbidopa-Levodopa 25-100 mg three times daily
- Rasagiline 1 mg every morning
- Metoprolol succinate 50 mg daily
- Aspirin 81 mg daily
- Tamsulosin 0.4 mg nightly at bedtime

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 72
- Mother: Ischemic stroke at age 78
- One older brother with hypertension
- No family history of Parkinson disease or psychiatric illness

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former occasional use (1-2 drinks per month), quit 6 months ago. Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 145/90 mmHg, and his pulse is 78 beats per minute and regular.

PHYSICAL EXAMINATION:
On examination, his left pupil is non-reactive to light, and there is a relative afferent pupillary defect. Fundoscopic examination of the left eye reveals a pale retina with a cherry-red spot at the macula. His right eye examination is normal. Carotid auscultation reveals no bruits.

IMAGING:
An urgent non-contrast computed tomography (CT) scan of the head shows no acute intracranial abnormalities.
