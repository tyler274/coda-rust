# synthetic_hospital patient 2244

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-25 (Sun) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-11-17 (Mon) | outpatient | Neurology |
| 2 | 2021-01-15 | 2026-06-01 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired accountant",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father: ischemic stroke at age 74",
  "Mother: hypertension",
  "Brother: coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 8 years ago, 35 pack-year history",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Cataract surgery (2021)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Acute ischemic stroke", "Vascular dementia"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-05-25, ed, Emergency Medicine

Chief complaint: Dragging his left foot while walking

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dragging his left foot while walking

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department after his wife noticed he was dragging his left foot while walking. He denies pain but says he feels like he “can’t lift his leg well.”

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia

He has a history of hypertension and hyperlipidemia. He has no prior history of stroke or TIA.

PAST SURGICAL HISTORY:
- Cataract surgery (2021)

MEDICATIONS:
He takes lisinopril and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: ischemic stroke at age 74
- Mother: hypertension
- Brother: coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 35 pack-year history. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On examination, he is alert and oriented. Cranial nerves are intact, and strength is 5/5 in the upper extremities and right leg. Strength is 3/5 in the left lower extremity with slightly increased tone. There is no sensory loss, and coordination and reflexes are normal in the arms.

IMAGING:
Non-contrast CT of the head confirms the diagnosis.

## Encounter 1: source 2020-07-15, practice 2025-11-17, outpatient, Neurology

Chief complaint: Sudden change in behavior and cognitive function over the past week; mild left-sided weakness

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Raj Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden change in behavior and cognitive function over the past week; mild left-sided weakness

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with hypertension and hyperlipidemia who returns for neurologic evaluation six months following an acute ischemic stroke. He presents to the clinic with his daughter, who reports that her father experienced a sudden change in behavior and cognitive function over the past week. She notes that he has become forgetful, is easily distracted, and has difficulty planning his day, which is uncharacteristic of his usual organized nature. The patient also complains of mild left-sided weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute ischemic stroke (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract surgery (2021)

MEDICATIONS:
Lisinopril and atorvastatin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: ischemic stroke at age 74
- Mother: hypertension
- Brother: coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 35 pack-year history. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears apathetic. His speech is fluent with no evidence of aphasia. Motor examination reveals mild left-sided drift. Cranial nerves are intact, and sensory examination is normal.

IMAGING:
A non-contrast head CT scan shows a small, well-circumscribed area of hypodensity in the right frontal lobe.

## Encounter 2: source 2021-01-15, practice 2026-06-01, ed, Emergency Medicine

Chief complaint: Sudden onset dizziness, nausea, and unsteadiness on his feet

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset dizziness, nausea, and unsteadiness on his feet

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with hypertension, hyperlipidemia, prior ischemic stroke, and vascular dementia who returns to the emergency department with dizziness. He denies any recent head trauma or loss of consciousness. He reports that the room seems to be spinning around him and that the dizziness does not improve when he lies still or tries to focus on an object. He also mentions a mild headache that started around the same time as the dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute ischemic stroke (diagnosed 2020-01-15)
- Vascular dementia (diagnosed 2020-07-15)
- Hypertension
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Cataract surgery (2021)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: ischemic stroke at age 74
- Mother: hypertension
- Brother: coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 35 pack-year history. Alcohol: Occasional (1-2 drinks per month). Occupation: Retired accountant.

VITAL SIGNS:
His blood pressure is 160/90 mmHg, heart rate is 78 bpm, and he is afebrile.

PHYSICAL EXAMINATION:
On examination, you note horizontal nystagmus that does not diminish with visual fixation and a slight slurring of speech.
Neurological examination reveals dysmetria on finger-to-nose testing on the right side. There is no evidence of facial droop, arm drift, or speech deficits suggestive of a classic stroke.
