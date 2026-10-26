# synthetic_hospital patient 2253

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2020-07-15 | 2025-06-08 (Sun) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2025-12-10 (Wed) | ed | Emergency Medicine |
| 5 | 2021-07-15 | 2026-06-05 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired accountant",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Ischemic stroke at age 74",
  "Older brother: Hypertension and hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-year history, quit 8 years ago",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 22)",
  "Left knee arthroscopy (age 48)"
 ],
 "chronic_conditions": [
  "Essential Hypertension",
  "Hyperlipidemia",
  "Type 2 Diabetes Mellitus",
  "Obesity"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Right middle cerebral artery stroke", "Right Ventricular Myocardial Infarction", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 1: source 2020-07-15, practice 2025-06-08, ed, Emergency Medicine

Chief complaint: Severe chest pain

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe chest pain

HISTORY OF PRESENT ILLNESS:
This 65-year-old male with a history of essential hypertension, hyperlipidemia, type 2 diabetes mellitus, and obesity presents to the emergency department with severe chest pain that began 2 hours ago while he was resting. The pain is described as a constant, heavy sensation located substernally, radiating to the right shoulder. He also reports feeling lightheaded and has noted an episode of syncope prior to arrival.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential Hypertension
- Hyperlipidemia
- Type 2 Diabetes Mellitus
- Obesity

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Left knee arthroscopy (age 48)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Ischemic stroke at age 74
- Older brother: Hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 8 years ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired accountant.

VITAL SIGNS:
A blood pressure of 85/50 mmHg, heart rate of 110 beats per minute

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, with jugular venous distension, and clear lungs. There is no pedal edema.

LABORATORY DATA:
Laboratory values are as follows:

- Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 50 U/L (normal 0-24 U/L)
- B-type natriuretic peptide (BNP): 120 pg/mL (normal <100 pg/mL)

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevation in leads II, III, and aVF, with additional ST-segment elevation noted in the right-sided precordial lead V4R.

## Encounter 3: source 2021-01-15, practice 2025-12-10, ed, Emergency Medicine

Chief complaint: Severe epigastric pain and nausea

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric pain and nausea

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with essential hypertension, hyperlipidemia, type 2 diabetes, and obesity, who had an inferior wall STEMI in July 2020, presents with a 2-hour history of severe epigastric pain and nausea. He describes the pain as a constant, heavy sensation that does not radiate and is not relieved by antacids. He also reports feeling lightheaded.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2020-07-15)
- Essential Hypertension
- Hyperlipidemia
- Type 2 Diabetes Mellitus
- Obesity

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Left knee arthroscopy (age 48)

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Ischemic stroke at age 74
- Older brother: Hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 8 years ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired accountant.

VITAL SIGNS:
Blood pressure is 90/60 mmHg, heart rate is 55 bpm, and he is afebrile

PHYSICAL EXAMINATION:
His jugular venous pressure is elevated, and there is clear evidence of peripheral edema. Lung auscultation is clear, and heart sounds are distant but without murmurs.

LABORATORY DATA:
Troponin I: 8.0 ng/mL (normal <0.04 ng/mL)
Creatinine: 1.2 mg/dL (normal 0.6-1.3 mg/dL)
Potassium: 4.5 mEq/L (normal 3.5-5.0 mEq/L)
Hemoglobin: 14 g/dL (normal 13.5-17.5 g/dL)

OTHER STUDIES:
An ECG shows ST-segment elevation in leads II, III, and aVF.

## Encounter 5: source 2021-07-15, practice 2026-06-05, ed, Emergency Medicine

Chief complaint: Difficulty walking and urinary incontinence

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Difficulty walking and urinary incontinence

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with hypertension, hyperlipidemia, diabetes, obesity, and prior myocardial infarctions, brought to the emergency department by his wife after she noticed that he was having difficulty walking and was unable to control his urination. His symptoms started suddenly about 2 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2020-07-15)
- Right Ventricular Myocardial Infarction (diagnosed 2021-01-15)
- Essential Hypertension
- Hyperlipidemia
- Type 2 Diabetes Mellitus
- Obesity

His past medical history is significant for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Appendectomy (age 22)
- Left knee arthroscopy (age 48)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg nightly
- Metformin 1000 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Ischemic stroke at age 74
- Older brother: Hypertension and hyperlipidemia

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-year history, quit 8 years ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired accountant.

VITAL SIGNS:
The patient's blood pressure is 160/90 mmHg, heart rate is 78 beats per minute, and the rest of the vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient is alert and oriented. His speech is fluent with no evidence of aphasia. Cranial nerve examination is normal. Motor examination reveals left lower extremity weakness with a strength of 3/5, while the left upper extremity has a strength of 4/5. Sensory examination shows decreased sensation to light touch and pinprick in the left lower extremity. The right upper and lower extremities are normal on motor and sensory examination. Reflexes are 2+ and symmetric. Babinski sign is upgoing on the left side. There is no evidence of cerebellar dysfunction.

IMAGING:
A non-contrast head CT scan shows no hemorrhage or mass effect.
