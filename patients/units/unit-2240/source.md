# synthetic_hospital patient 2240

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-12 (Wed) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2021-08-02 (Mon) | outpatient | Gastroenterology |
| 2 | 2020-10-15 | 2022-01-31 (Mon) | outpatient | Urology |
| 3 | 2022-01-15 | 2023-04-30 (Sun) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2025-05-12 (Mon) | ed | Neurosurgery |
| 5 | 2024-07-15 | 2025-11-05 (Wed) | icu | Vascular Surgery |
| 6 | 2024-10-15 | 2026-02-12 (Thu) | icu | Vascular Surgery |
| 7 | 2025-01-15 | 2026-05-09 (Sat) | icu | Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Industrial dye manufacturing technician",
 "alcohol_use": "Chronic alcohol use disorder with heavy daily consumption (approximately half bottle of whiskey or equivalent)",
 "family_history": [
  "Father with abdominal aortic aneurysm",
  "Mother with hypertension",
  "Family history of alcohol use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 40 pack-year history (1 pack per day)",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "No previous abdominal surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Alcohol use disorder",
  "Peripheral arterial disease",
  "Paget disease of bone",
  "Upper urinary tract urothelial carcinoma",
  "Niacin deficiency (Pellagra)"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Ruptured abdominal aortic aneurysm", "Paget disease of bone", "Peripheral Arterial Disease with intermittent claudication", "Acute subdural hematoma", "Upper urinary tract urothelial carcinoma", "Pellagra (Niacin deficiency)"]
Comorbidities: ["Alcohol use disorder", "Tobacco use disorder", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-05-12, outpatient, Cardiology

Chief complaint: Cramping pain in calves when walking that resolves with rest.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Cramping pain in calves when walking that resolves with rest.

HISTORY OF PRESENT ILLNESS:
He reports a cramping pain in his calves when walking that resolves with rest. He denies any chest pain, palpitations, or dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease
- Paget disease of bone
- Upper urinary tract urothelial carcinoma
- Niacin deficiency (Pellagra)

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
His medications include hydrochlorothiazide and amlodipine.

Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
History of smoking

VITAL SIGNS:
His blood pressure is 142/88 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
His pulses are diminished in both feet. There are no skin ulcers or temperature differences between his legs. His femoral pulses are palpable but weak.

OTHER STUDIES:
Ankle-brachial index (ABI) is performed with the following results:

Right Ankle-Brachial Index: 0.65
Left Ankle-Brachial Index: 0.70

## Encounter 1: source 2020-04-15, practice 2021-08-02, outpatient, Gastroenterology

Chief complaint: Progressive skin changes, frequent watery diarrhea, and cognitive symptoms including forgetfulness and irritability.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Emily Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive skin changes, frequent watery diarrhea, and cognitive symptoms including forgetfulness and irritability.

HISTORY OF PRESENT ILLNESS:
This 68-year-old male with alcohol use disorder presents to Gastroenterology following a Cardiology visit for claudication. He reports a history of chronic alcohol use and has had frequent, watery, non-bloody diarrhea along with poor appetite and unintentional weight loss over the past 2 months. He has also become increasingly forgetful and irritable, per family.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease
- Paget disease of bone
- Upper urinary tract urothelial carcinoma
- Niacin deficiency (Pellagra)

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Amlodipine 5 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father died of esophageal cancer at age 70.

SOCIAL HISTORY:
He smokes one pack of cigarettes daily and drinks approximately half a bottle of whiskey most days.

VITAL SIGNS:
Vital signs show temperature 37.1°C (98.8°F), pulse 84/min, blood pressure 126/78 mm Hg, and respiratory rate 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals a thin man appearing older than stated age, with normal cardiopulmonary and abdominal exams. An image of the patient's neck is shown below.

## Encounter 2: source 2020-10-15, practice 2022-01-31, outpatient, Urology

Chief complaint: Intermittent blood in urine and vague flank discomfort with unintentional weight loss over 3 months.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Raj Patel, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent blood in urine and vague flank discomfort with unintentional weight loss over 3 months.

HISTORY OF PRESENT ILLNESS:
The patient, with history of peripheral arterial disease and pellagra diagnosed earlier this year, presents with urinary symptoms. He describes the pain as dull and non-radiating, without any specific aggravating or relieving factors. He denies fever, dysuria, or recent urinary tract infections. His appetite has decreased slightly, and he has unintentionally lost about 4 kg in the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Pellagra (Niacin deficiency) (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease
- Paget disease of bone
- Upper urinary tract urothelial carcinoma

His past medical history is notable for hypertension,.

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
He takes he has been taking lisinopril regularly.

Home Medications:
- Hydrochlorothiazide 25 mg daily
- Amlodipine 5 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
He has smoked a pack of cigarettes daily for over 40 years. He works in an industrial dye manufacturing facility and frequently handles aromatic chemicals without consistent use of protective equipment.

VITAL SIGNS:
On examination, his vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical exam is unremarkable except for mild costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows hematuria without signs of infection.

IMAGING:
Further evaluation with imaging and cystoscopy reveals a mass in the renal calyceal system.

PATHOLOGY:
A gross pathology photograph from nephrectomy confirms a pale, friable tumor centered in the calyces with extension into the renal pelvis , as shown.

## Encounter 3: source 2022-01-15, practice 2023-04-30, ed, Emergency Medicine

Chief complaint: Fall in bathroom with brief loss of consciousness; partner notes progressive clumsiness and hat no longer fits properly.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fall in bathroom with brief loss of consciousness; partner notes progressive clumsiness and hat no longer fits properly.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with a history of Paget disease of bone, upper urinary tract urothelial carcinoma, pellagra, and peripheral arterial disease who presents to the emergency department following a fall. He reports that he tripped getting out of the shower and hit the back of his head on the tile. His partner mentions that although he did not have a seizure, she has noticed he has been increasingly clumsy over the past few months. The patient adds that his favorite hat no longer fits the way it used to, and he has had some occasional left-sided hearing loss. He denies headache, diplopia, or focal weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Pellagra (Niacin deficiency) (diagnosed 2020-04-15)
- Upper urinary tract urothelial carcinoma (diagnosed 2020-10-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease
- Paget disease of bone

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Amlodipine 5 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
He has a 40-pack-year smoking history and drinks one glass of wine with dinner.

VITAL SIGNS:
His temperature is 37.0°C (98.6°F), blood pressure is 142/80 mm Hg, and pulse is 76/min.

PHYSICAL EXAMINATION:
Physical examination reveals a prominent occipital ridge and mild sensorineural hearing loss in the left ear.

IMAGING:
Non-contrast CT of the head shows no evidence of intracranial bleeding but does reveal irregular thickening and patchy sclerosis of the occipital bone with mixed radiolucent and radiodense areas.

## Encounter 4: source 2024-01-15, practice 2025-05-12, ed, Neurosurgery

Chief complaint: Confusion and headache after falling down stairs earlier in the day.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Wilson, MD
Department: Neurosurgery
Visit Type: ed

CHIEF COMPLAINT:
Confusion and headache after falling down stairs earlier in the day.

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Paget disease and prior falls who presents to the emergency department with a one-hour history of confusion and headache. His wife reports that he fell down the stairs at home earlier in the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Pellagra (Niacin deficiency) (diagnosed 2020-04-15)
- Upper urinary tract urothelial carcinoma (diagnosed 2020-10-15)
- Paget disease of bone (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease

He has a history of chronic alcoholism and hypertension.

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Chronic alcohol use disorder with heavy daily consumption (approximately half bottle of whiskey or equivalent). Occupation: Industrial dye manufacturing technician.

PHYSICAL EXAMINATION:
On examination, he is drowsy but arousable, with a Glasgow Coma Scale score of 14. His pupils are equal and reactive, and there is no lateralizing neurological deficit.

IMAGING:
A noncontrast head CT scan is performed and reveals a crescent-shaped hyperdensity that crosses suture lines over the left cerebral hemisphere.

## Encounter 5: source 2024-07-15, practice 2025-11-05, icu, Vascular Surgery

Chief complaint: Sudden-onset severe tearing abdominal pain radiating to the lower back.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Zhang, MD
Department: Vascular Surgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden-onset severe tearing abdominal pain radiating to the lower back.

HISTORY OF PRESENT ILLNESS:
The patient has peripheral arterial disease. He describes the pain as tearing and radiating to the lower back.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Pellagra (Niacin deficiency) (diagnosed 2020-04-15)
- Upper urinary tract urothelial carcinoma (diagnosed 2020-10-15)
- Paget disease of bone (diagnosed 2022-01-15)
- Acute subdural hematoma (diagnosed 2024-01-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease

His past medical history is significant for hypertension and chronic alcohol use.

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Amlodipine 5 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
He has smoked one pack of cigarettes daily for 40 years.

VITAL SIGNS:
His blood pressure is 92/56 mm Hg, pulse is 112/min

PHYSICAL EXAMINATION:
He appears diaphoretic. Abdominal palpation reveals a tender, pulsatile mass in the epigastrium.

PLAN:
He is taken for emergent imaging and vascular surgery consultation.

## Encounter 6: source 2024-10-15, practice 2026-02-12, icu, Vascular Surgery

Chief complaint: Sudden severe abdominal pain radiating to back with lightheadedness and brief syncopal episode.

ENCOUNTER NOTE
Date: 2024-10-15
Provider: Dr. Lisa Zhang, MD
Department: Vascular Surgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden severe abdominal pain radiating to back with lightheadedness and brief syncopal episode.

HISTORY OF PRESENT ILLNESS:
The patient returns three months following his prior ruptured AAA. He describes the pain as a tearing sensation. He also reports feeling light-headed and had a brief syncopal episode at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Pellagra (Niacin deficiency) (diagnosed 2020-04-15)
- Upper urinary tract urothelial carcinoma (diagnosed 2020-10-15)
- Paget disease of bone (diagnosed 2022-01-15)
- Acute subdural hematoma (diagnosed 2024-01-15)
- Ruptured abdominal aortic aneurysm (diagnosed 2024-07-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Hydrochlorothiazide 25 mg daily
- Amlodipine 5 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Chronic alcohol use disorder with heavy daily consumption (approximately half bottle of whiskey or equivalent). Occupation: Industrial dye manufacturing technician.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, pulse is 120 beats per minute

PHYSICAL EXAMINATION:
On examination, he appears diaphoretic and pale. Abdominal examination reveals a pulsatile mass just above the umbilicus. There is no rebound tenderness or guarding.

LABORATORY DATA:
Hemoglobin is 10 g/dL (decreased from a baseline of 14 g/dL six months ago), hematocrit is 30%, and serum lactate is elevated.

IMAGING:
A bedside ultrasound shows an abdominal aortic aneurysm with evidence of free fluid in the abdomen.

## Encounter 7: source 2025-01-15, practice 2026-05-09, icu, Critical Care

Chief complaint: Collapse at home after complaining of tearing sensation in the abdomen.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Collapse at home after complaining of tearing sensation in the abdomen.

HISTORY OF PRESENT ILLNESS:
The patient returns following prior similar presentations. His wife reports that he collapsed at home after complaining of a tearing sensation in his abdomen.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral Arterial Disease with intermittent claudication (diagnosed 2020-01-15)
- Pellagra (Niacin deficiency) (diagnosed 2020-04-15)
- Upper urinary tract urothelial carcinoma (diagnosed 2020-10-15)
- Paget disease of bone (diagnosed 2022-01-15)
- Acute subdural hematoma (diagnosed 2024-01-15)
- Ruptured abdominal aortic aneurysm (diagnosed 2024-07-15)
- Hypertension
- Hyperlipidemia
- Alcohol use disorder
- Peripheral arterial disease

PAST SURGICAL HISTORY:
- No previous abdominal surgeries

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Amlodipine 5 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with abdominal aortic aneurysm
- Mother with hypertension
- Family history of alcohol use disorder

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (1 pack per day). Alcohol: Chronic alcohol use disorder with heavy daily consumption (approximately half bottle of whiskey or equivalent). Occupation: Industrial dye manufacturing technician.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 120 beats per minute

PHYSICAL EXAMINATION:
On examination, he is diaphoretic and pale. His abdomen is distended with a pulsatile mass palpable in the periumbilical region. His extremities are cool to the touch with delayed capillary refill.

IMAGING:
Bedside focused assessment with sonography for trauma (FAST) reveals a large aneurysm with evidence of free fluid in the abdomen.
