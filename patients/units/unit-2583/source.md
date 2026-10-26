# synthetic_hospital patient 2583

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 20 at first encounter, sex M, White, Private insurance via parental plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-13 (Mon) | outpatient | Cardiology |
| 1 | 2021-01-15 | 2024-03-06 (Wed) | ed | Emergency Medicine |
| 2 | 2023-01-15 | 2026-03-06 (Fri) | inpatient | Neurosurgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance via parental plan",
 "occupation": "University student",
 "alcohol_use": "Occasional social use",
 "family_history": [
  "Sudden cardiac death in father at age 41",
  "Hypertrophic cardiomyopathy in paternal uncle"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Craniotomy with evacuation of epidural hematoma"
 ],
 "chronic_conditions": [
  "Hypertrophic obstructive cardiomyopathy"
 ],
 "age_at_first_encounter": 20
}
```

Primary diagnoses: ["Traumatic epidural hematoma", "Hypertrophic obstructive cardiomyopathy", "Loss of consciousness (syncope)"]
Comorbidities: ["Loss of consciousness (syncope)"]

## Encounter 0: source 2020-01-15, practice 2023-03-13, outpatient, Cardiology

Chief complaint: Collapsing during track practice

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Collapsing during track practice

HISTORY OF PRESENT ILLNESS:
He reports brief chest tightness before losing consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertrophic obstructive cardiomyopathy

He denies prior medical issues.

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of epidural hematoma

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father died suddenly at age 39

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: University student.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
Physical examination reveals a systolic murmur at the left lower sternal border.

IMAGING:
Echocardiography shows asymmetric septal hypertrophy with systolic anterior motion of the mitral valve. No structural valvular abnormalities are identified.

## Encounter 1: source 2021-01-15, practice 2024-03-06, ed, Emergency Medicine

Chief complaint: Recurrent episodes of transient loss of consciousness

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of transient loss of consciousness

HISTORY OF PRESENT ILLNESS:
Patient with hypertrophic obstructive cardiomyopathy, originally diagnosed following track practice collapse, returns with recurrent syncope. He reports that these episodes are often preceded by feelings of nausea, warmth, and lightheadedness. During an episode earlier today, his friends observed him to be pale and diaphoretic before he slumped to the ground.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of epidural hematoma

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Sudden cardiac death in father at age 41
- Hypertrophic cardiomyopathy in paternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: University student.

VITAL SIGNS:
His blood pressure is 120/75 mmHg, heart rate is 70 bpm

PHYSICAL EXAMINATION:
He is alert and oriented

OTHER STUDIES:
An ECG performed while the patient is asymptomatic shows normal sinus rhythm with no abnormalities. The patient has an episode of syncope while on the cardiac monitor, which shows sinus bradycardia followed by a brief period of asystole before returning to a normal sinus rhythm.

## Encounter 2: source 2023-01-15, practice 2026-03-06, inpatient, Neurosurgery

Chief complaint: Severe headache after motorcycle accident

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Elena Vasquez, MD
Department: Neurosurgery
Visit Type: inpatient

CHIEF COMPLAINT:
Severe headache after motorcycle accident

HISTORY OF PRESENT ILLNESS:
The patient is a 23-year-old male with hypertrophic obstructive cardiomyopathy and prior syncope, who is brought to the emergency department after being involved in a motorcycle accident. He was not wearing a helmet and was found unconscious at the scene. On arrival, he regains consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertrophic obstructive cardiomyopathy (diagnosed 2020-01-15)
- Loss of consciousness (syncope) (diagnosed 2021-01-15)

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of epidural hematoma

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Sudden cardiac death in father at age 41
- Hypertrophic cardiomyopathy in paternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: University student.

VITAL SIGNS:
Blood pressure 140/90 mmHg, heart rate 100 beats per minute, respiratory rate 18 breaths per minute, and temperature 37.0°C (98.6°F)

PHYSICAL EXAMINATION:
On examination, he is alert and oriented, but appears restless. Neurological examination reveals no focal deficits.

IMAGING:
A CT scan of the head is performed and shows a hyperdense biconvex lesion that does not cross suture lines.
