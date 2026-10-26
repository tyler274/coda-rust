# synthetic_hospital patient 2657

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, Non-Hispanic White, University health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-24 (Wed) | outpatient | Cardiology |
| 1 | 2020-07-15 | 2026-07-03 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University health insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Maternal uncle with Wolff-Parkinson-White syndrome",
  "Paternal grandfather with atrial fibrillation"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO BID",
   "name": "Flecainide"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction, age 18"
 ],
 "chronic_conditions": [
  "Wolff-Parkinson-White Syndrome"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Atrial fibrillation", "Atrial Fibrillation with Wolff-Parkinson-White Syndrome"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-12-24, outpatient, Cardiology

Chief complaint: Routine follow-up for chronic atrial fibrillation and medication management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic atrial fibrillation and medication management

HISTORY OF PRESENT ILLNESS:
A 24-year-old male with a known history of Wolff-Parkinson-White (WPW) syndrome presents to the emergency department with palpitations and dizziness that started abruptly while playing basketball. He denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Wolff-Parkinson-White Syndrome

His past medical history is significant for several episodes of tachycardia that were managed with medication.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
Home Medications:
- Flecainide 50 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with Wolff-Parkinson-White syndrome
- Paternal grandfather with atrial fibrillation

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, his blood pressure is 100/60 mmHg, heart rate is 210 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
His heart rhythm is irregularly irregular, and there are no additional heart sounds or murmurs.

OTHER STUDIES:
An electrocardiogram (ECG) shows atrial fibrillation with a rapid ventricular response and evidence of pre-excitation.

## Encounter 1: source 2020-07-15, practice 2026-07-03, ed, Emergency Medicine

Chief complaint: Acute onset palpitations and dizziness while playing basketball

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset palpitations and dizziness while playing basketball

HISTORY OF PRESENT ILLNESS:
With Wolff-Parkinson-White Syndrome and atrial fibrillation, last evaluated on 2020-01-15, he presents to the emergency department with palpitations and dizziness that started abruptly while playing basketball. He denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Atrial fibrillation (diagnosed 2020-01-15)
- Wolff-Parkinson-White Syndrome

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
Home Medications:
- Flecainide 50 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with Wolff-Parkinson-White syndrome
- Paternal grandfather with atrial fibrillation

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, his heart rate is 180 beats per minute, blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
He appears anxious but not in distress. His lungs are clear to auscultation, and there is no jugular venous distention. His heart rhythm is regular with no murmurs, rubs, or gallops.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows a short PR interval, a wide QRS complex with a slurred upstroke (δ wave), and minimal ST-segment changes. There is no evidence of ischemia or infarction.
