# synthetic_hospital patient 2259

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-28 (Wed) | procedure | Vascular Surgery |
| 1 | 2020-07-15 | 2023-03-27 (Mon) | outpatient | Neurology |
| 2 | 2021-03-15 | 2023-11-11 (Sat) | ed | Emergency Medicine |
| 3 | 2021-11-15 | 2024-07-29 (Mon) | inpatient | Neurology |
| 4 | 2022-07-15 | 2025-03-14 (Fri) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2025-09-19 (Fri) | inpatient | Cardiology |
| 6 | 2023-07-15 | 2026-03-29 (Sun) | icu | Cardiology |
| 7 | 2023-08-15 | 2026-04-25 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired landscape architect",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Abdominal aortic aneurysm rupture at age 72",
  "Mother: Myocardial infarction at age 68",
  "Brother: Stroke at age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 45 pack-year history, quit at age 62",
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
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis",
  "Cardiac catheterization with percutaneous coronary intervention",
  "Right carotid endarterectomy"
 ],
 "chronic_conditions": [
  "Hypertension (poorly controlled)",
  "Hyperlipidemia",
  "Infrarenal abdominal aortic aneurysm (repaired)",
  "Carotid artery stenosis",
  "Peripheral artery disease",
  "History of cerebrovascular disease with bilateral involvement",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Atheroembolism (Cholesterol Embolism Syndrome)", "Diverticulosis of colon", "Sinus bradycardia with non-conducted premature atrial contractions", "Non-ST elevation myocardial infarction (NSTEMI)", "Transient ischemic attack due to symptomatic carotid artery stenosis", "Lacunar stroke", "Neuropathic pain syndrome", "Right Ventricular Myocardial Infarction"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension", "Abdominal aortic aneurysm"]

## Encounter 0: source 2020-01-15, practice 2022-09-28, procedure, Vascular Surgery

Chief complaint: Preoperative evaluation for elective resection of incidentally discovered infrarenal abdominal aortic aneurysm

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Vascular Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Preoperative evaluation for elective resection of incidentally discovered infrarenal abdominal aortic aneurysm

HISTORY OF PRESENT ILLNESS:
He has never undergone colonoscopy and denies prior gastrointestinal bleeding, changes in bowel habits, abdominal pain, or unexplained weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

He has a history of hypertension and hyperlipidemia but is otherwise in good health.

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily at bedtime
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

PHYSICAL EXAMINATION:
Intraoperatively, the surgeon notes multiple small, saclike protrusions of mucosa and submucosa through the muscular layer of the colon wall. These protrusions are most numerous in one segment of the large intestine and are separated by areas of normal mucosa. The serosal surface shows no signs of active inflammation or perforation.

ASSESSMENT AND PLAN:
The patient's postoperative course is uneventful, and he remains asymptomatic.

## Encounter 1: source 2020-07-15, practice 2023-03-27, outpatient, Neurology

Chief complaint: Two-month history of intermittent severe burning pain on the left side of the body triggered by light touch

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of intermittent severe burning pain on the left side of the body triggered by light touch

HISTORY OF PRESENT ILLNESS:
This 67-year-old with prior AAA repair and CVA history returns. He describes the pain as "lightning-like" and reports that it is triggered by the light touch of his clothes against his skin.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

He has a history of hypertension and hyperlipidemia and experienced a sudden onset of left-sided numbness 3 months ago, which resolved almost completely within a few weeks.

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
His current medications include lisinopril, atorvastatin, and aspirin.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

PHYSICAL EXAMINATION:
On physical examination, there is no motor weakness, but he exhibits hyperesthesia to light touch and pinprick on the left side of his face, arm, and leg. His deep tendon reflexes are normal, and there are no cerebellar signs.

IMAGING:
His recent MRI of the brain with diffusion-weighted imaging showed a small, well-circumscribed area of hyperintensity in the right posterolateral thalamus, consistent with an old lacunar infarct.

## Encounter 2: source 2021-03-15, practice 2023-11-11, ed, Emergency Medicine

Chief complaint: Sudden onset of weakness in right arm and difficulty speaking that resolved completely within one hour

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of weakness in right arm and difficulty speaking that resolved completely within one hour

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with symptomatic carotid artery stenosis, poorly controlled hypertension, and bilateral cerebrovascular disease who presents to the emergency department. He has previously undergone infrarenal abdominal aortic aneurysm repair and was evaluated by Neurology in 2020 for neuropathic pain. He reports sudden onset of weakness in his right arm and difficulty speaking that resolved completely within an hour.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Neuropathic pain syndrome (diagnosed 2020-07-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
He has been compliant with his medication regimen, which includes lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

VITAL SIGNS:
His blood pressure in the ED is 145/90 mmHg

PHYSICAL EXAMINATION:
On physical examination, a bruit is auscultated over the left carotid artery.

LABORATORY DATA:
His cholesterol panel from two weeks ago showed an LDL of 98 mg/dL

IMAGING:
A Doppler ultrasound of the carotid arteries reveals 75% stenosis of the left internal carotid artery.

## Encounter 3: source 2021-11-15, practice 2024-07-29, inpatient, Neurology

Chief complaint: Sudden onset of weakness in right arm and leg without associated headache or speech difficulty

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. David Martinez, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden onset of weakness in right arm and leg without associated headache or speech difficulty

HISTORY OF PRESENT ILLNESS:
This 68-year-old with prior transient ischemic attack presents. He reports no associated headache, vision changes, or difficulty speaking.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Neuropathic pain syndrome (diagnosed 2020-07-15)
- Transient ischemic attack due to symptomatic carotid artery stenosis (diagnosed 2021-03-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
He takes hydrochlorothiazide and amlodipine.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily at bedtime
- Lisinopril 20 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
He admits to being noncompliant with his medications and diet.

VITAL SIGNS:
On examination, his blood pressure is 190/100 mmHg.

PHYSICAL EXAMINATION:
Neurological examination reveals grade 3/5 strength in the right upper and lower extremities with normal muscle tone and reflexes. Sensation is intact, and there are no signs of aphasia or apraxia.

LABORATORY DATA:
Laboratory Values:
- WBC: 7.2 x 10^3/uL
- Hemoglobin: 14.1 g/dL
- Platelets: 250 x 10^3/uL
- Serum glucose: 110 mg/dL
- Serum creatinine: 1.0 mg/dL
- LDL cholesterol: 145 mg/dL
- HbA1c: 6.0%

IMAGING:
A non-contrast head CT scan shows no hemorrhage or mass effect.

## Encounter 4: source 2022-07-15, practice 2025-03-14, ed, Emergency Medicine

Chief complaint: Syncopal episode while gardening with preceding lightheadedness and rapid recovery without confusion

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Syncopal episode while gardening with preceding lightheadedness and rapid recovery without confusion

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with cerebrovascular disease, coronary artery disease, and prior transient ischemic attack and lacunar stroke presents with syncope. He reports feeling lightheaded before losing consciousness for a few seconds. His wife, who witnessed the event, states that he did not exhibit any convulsive movements, and he regained consciousness quickly without any confusion. He denies chest pain, palpitations, or shortness of breath prior to the event.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Neuropathic pain syndrome (diagnosed 2020-07-15)
- Transient ischemic attack due to symptomatic carotid artery stenosis (diagnosed 2021-03-15)
- Lacunar stroke (diagnosed 2021-11-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

VITAL SIGNS:
On examination, his blood pressure is 130/80 mmHg, heart rate is 55 bpm, and he is afebrile.

PHYSICAL EXAMINATION:
His cardiovascular examination is unremarkable with no murmurs, rubs, or gallops, and his neurological examination is normal.

OTHER STUDIES:
An ECG is performed and shows sinus bradycardia with occasional sinus pauses lasting up to 3 seconds.

## Encounter 5: source 2023-01-15, practice 2025-09-19, inpatient, Cardiology

Chief complaint: Chest pressure radiating to left arm with associated shortness of breath for two hours

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Chest pressure radiating to left arm with associated shortness of breath for two hours

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with coronary artery disease, prior infrarenal aortic aneurysm repair, and cerebrovascular disease presents with chest pain that started 2 hours ago. The pain is described as a pressure-like sensation in the center of his chest, radiating to left arm, and is associated with shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Neuropathic pain syndrome (diagnosed 2020-07-15)
- Transient ischemic attack due to symptomatic carotid artery stenosis (diagnosed 2021-03-15)
- Lacunar stroke (diagnosed 2021-11-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2022-07-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
He takes atorvastatin and lisinopril.

Home Medications:
- Aspirin 81 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

VITAL SIGNS:
Temperature is 37.0°C (98.6°F), blood pressure is 150/90 mmHg, pulse is 85/min, and respiratory rate is 18/min

PHYSICAL EXAMINATION:
Regular rhythm without murmurs, rubs, or gallops, and his lungs are clear to auscultation

LABORATORY DATA:
Troponin I level is elevated at 0.5 ng/mL (normal <0.04 ng/mL)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment depression in leads II, III, and aVF

ASSESSMENT AND PLAN:
Non-ST elevation myocardial infarction (NSTEMI)

Managed with aspirin, a beta blocker, and high-dose statin therapy. He undergoes successful placement of a drug-eluting stent in the right coronary artery.

## Encounter 6: source 2023-07-15, practice 2026-03-29, icu, Cardiology

Chief complaint: Severe chest pressure radiating to left arm associated with nausea and diaphoresis

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Lisa Wang, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Severe chest pressure radiating to left arm associated with nausea and diaphoresis

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with coronary artery disease, cerebrovascular disease, and prior myocardial infarction. He presents to the emergency department with a 2-hour history of severe chest pain radiating to his left arm, describing the pain as a pressure-like sensation associated with nausea and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Neuropathic pain syndrome (diagnosed 2020-07-15)
- Transient ischemic attack due to symptomatic carotid artery stenosis (diagnosed 2021-03-15)
- Lacunar stroke (diagnosed 2021-11-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2022-07-15)
- Non-ST elevation myocardial infarction (NSTEMI) (diagnosed 2023-01-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily at bedtime
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 50 beats per minute

PHYSICAL EXAMINATION:
He appears diaphoretic and in distress. Physical examination reveals a jugular venous pressure of 6 cm H2O and clear lung fields.

LABORATORY DATA:
- Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 50 U/L (normal 0-24 U/L)
- Total creatine kinase: 600 U/L (normal 38-174 U/L)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevations in leads II, III, and aVF, and ST-segment depressions in leads V1 and V2.

## Encounter 7: source 2023-08-15, practice 2026-04-25, ed, Emergency Medicine

Chief complaint: Acute painful purple discoloration and coldness of right foot that began earlier in the day

ENCOUNTER NOTE
Date: 2023-08-15
Provider: Dr. Emily Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute painful purple discoloration and coldness of right foot that began earlier in the day

HISTORY OF PRESENT ILLNESS:
The patient with history of aortic aneurysm repair and recent myocardial infarction presents with acute lower extremity symptoms. He reports that the foot initially felt cold and numb shortly after waking up, but over the past several hours, the toes became more tender and purple in appearance. He denies fever, chest pain, shortness of breath, or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis of colon (diagnosed 2020-01-15)
- Neuropathic pain syndrome (diagnosed 2020-07-15)
- Transient ischemic attack due to symptomatic carotid artery stenosis (diagnosed 2021-03-15)
- Lacunar stroke (diagnosed 2021-11-15)
- Sinus bradycardia with non-conducted premature atrial contractions (diagnosed 2022-07-15)
- Non-ST elevation myocardial infarction (NSTEMI) (diagnosed 2023-01-15)
- Right Ventricular Myocardial Infarction (diagnosed 2023-07-15)
- Hypertension (poorly controlled)
- Hyperlipidemia
- Infrarenal abdominal aortic aneurysm (repaired)
- Carotid artery stenosis
- Peripheral artery disease
- History of cerebrovascular disease with bilateral involvement
- Coronary artery disease

He has a long-standing history of poorly controlled hypertension and hyperlipidemia, and he was diagnosed with an infrarenal abdominal aortic aneurysm (AAA) two years ago.

PAST SURGICAL HISTORY:
- Infrarenal abdominal aortic aneurysm open repair with incidental finding of colonic diverticulosis
- Cardiac catheterization with percutaneous coronary intervention
- Right carotid endarterectomy

MEDICATIONS:
His medications include aspirin and atorvastatin.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm rupture at age 72
- Mother: Myocardial infarction at age 68
- Brother: Stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, 45 pack-year history, quit at age 62. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired landscape architect.

PHYSICAL EXAMINATION:
There is diminished sensation over the toes, but palpable dorsalis pedis and posterior tibial pulses are intact. The left foot appears normal. No skin ulcers or signs of infection are seen.

IMAGING:
A clinical image of the right foot is shown below.
