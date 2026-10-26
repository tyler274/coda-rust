# synthetic_hospital patient 2670

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Non-Hispanic White, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-10 (Sat) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2026-01-06 (Tue) | inpatient | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Office administrator",
 "alcohol_use": "None",
 "family_history": [
  "Father with generalized seizure disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "500 mg orally twice daily",
   "name": "Valproic acid"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Epilepsy"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Hyperchloremic metabolic acidosis due to excess normal saline infusion", "Epilepsy"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2025-05-10, ed, Emergency Medicine

Chief complaint: is brought to the emergency department by ambulance after an episode of sudden loss of consciousness at work.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He is brought to the emergency department by ambulance after an episode of sudden loss of consciousness at work.

HISTORY OF PRESENT ILLNESS:
His colleagues report that he experienced convulsions for approximately 2 minutes. On regaining consciousness, he was disoriented and drowsy. The patient has a history of similar episodes in the past, but he has not sought medical attention previously. He denies any recent illness, drug use, or alcohol abuse.

PAST MEDICAL HISTORY:
Active Problem List:
- Epilepsy

MEDICATIONS:
Home Medications:
- Valproic acid 500 mg orally twice daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is significant for epilepsy in his mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable. He has a bruised tongue and a small laceration on his lower lip. His vital signs are stable, and neurological examination is nonfocal.

LABORATORY DATA:
A postictal serum prolactin level is elevated.
Laboratory values:
- Serum glucose: 98 mg/dL
- Serum sodium: 140 mEq/L
- Serum calcium: 9.2 mg/dL
- Complete blood count: within normal limits

OTHER STUDIES:
An electroencephalogram (EEG) shows sharp wave discharges in the temporal lobe.

## Encounter 1: source 2020-09-15, practice 2026-01-06, inpatient, Pulmonary & Critical Care

Chief complaint: found unresponsive at home

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Pulmonary & Critical Care
Visit Type: inpatient

CHIEF COMPLAINT:
Found unresponsive at home

HISTORY OF PRESENT ILLNESS:
The 35-year-old patient with epilepsy returns following a January 2020 seizure episode. Brought to the emergency department by ambulance. His roommate reports that the patient had a generalized tonic-clonic seizure lasting approximately 2 minutes, which resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Epilepsy (diagnosed 2020-01-15)

MEDICATIONS:
Compliant with his medication, valproic acid

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with generalized seizure disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

VITAL SIGNS:
Temperature 37.0°C (98.6°F), blood pressure 130/85 mmHg, pulse 88/min, respiratory rate 8/min, and oxygen saturation 92% on room air

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but arousable, with a Glasgow Coma Scale score of 13.

LABORATORY DATA:
Arterial blood gas analysis reveals:

- pH: 7.28
- PaCO2: 60 mmHg
- HCO3-: 29 mEq/L
- PaO2: 70 mmHg
