# synthetic_hospital patient 2176

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Private (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-28 (Thu) | outpatient | Neurology |
| 1 | 2021-07-15 | 2026-06-06 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private (Blue Cross Blue Shield)",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Hypertension",
  "Mother: Breast cancer (age 68)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy, age 8"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Brain tumor (frontal lobe)", "Increased intracranial pressure with cerebral edema"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-11-28, outpatient, Neurology

Chief complaint: worsening headaches over the past 3 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening headaches over the past 3 months

HISTORY OF PRESENT ILLNESS:
The headaches are described as a constant, dull ache with intermittent sharp pains, predominantly in the frontal region. They are more severe in the morning and occasionally wake him from sleep. His wife has noticed that he has become increasingly apathetic and withdrawn, showing little interest in his hobbies or social activities. He also experienced a generalized tonic-clonic seizure earlier today, which prompted the visit.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

MEDICATIONS:
He takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Breast cancer (age 68)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, he is alert but disoriented to time. Cranial nerve examination is normal. Motor examination reveals mild weakness in the right upper extremity. Fundoscopic examination shows bilateral papilledema.

IMAGING:
A non-contrast head CT scan reveals a mass lesion in the left frontal lobe with surrounding edema.

## Encounter 1: source 2021-07-15, practice 2026-06-06, ed, Emergency Medicine

Chief complaint: high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with a frontal lobe brain tumor diagnosed in 2020 who presents following a high-speed motor vehicle collision. Emergency medical services report that he was initially responsive but confused at the scene. During transport, his mental status worsened, and he became progressively less verbal.

PAST MEDICAL HISTORY:
Active Problem List:
- Brain tumor (frontal lobe) (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 8

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Breast cancer (age 68)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: High school mathematics teacher.

VITAL SIGNS:
His vital signs are: temperature 37.0°C (98.6°F), blood pressure 198/104 mmHg, heart rate 44/min, respiratory rate 7/min, and oxygen saturation 91% on room air.

PHYSICAL EXAMINATION:
On arrival, he is unresponsive to verbal commands but withdraws to painful stimuli. His pupils are asymmetric, with the left larger than the right and sluggishly reactive to light. The patient has multiple scalp abrasions and a large contusion over the left temporal region. Breath sounds are symmetric, and heart tones are normal. Peripheral pulses are intact, and extremities show no deformities. The patient is intubated for airway protection.

IMAGING:
A non-contrast CT scan of the head reveals a rapidly expanding extra-axial hemorrhage with midline shift.
