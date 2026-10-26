# synthetic_hospital patient 2641

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 20 at first encounter, sex M, Non-Hispanic White, Private insurance through parents.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-04 (Wed) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2026-05-28 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through parents",
 "occupation": "University student",
 "alcohol_use": "Occasional weekend consumption (2-3 drinks per occasion)",
 "family_history": [
  "Paternal uncle: Aortic dissection (age 42)",
  "Father: Hypertension",
  "Mother: Hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg by mouth twice daily",
   "name": "Metoprolol tartrate"
  }
 ],
 "surgical_history": [
  "Emergent ascending aortic replacement and hemiarch repair (age 20)"
 ],
 "chronic_conditions": [
  "Primary hypertension",
  "Suspected Marfan syndrome (genetic evaluation pending)"
 ],
 "age_at_first_encounter": 20
}
```

Primary diagnoses: ["Type A Aortic Dissection", "Medication noncompliance"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-06-04, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine health check-up

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine health check-up

HISTORY OF PRESENT ILLNESS:
Despite this, his blood pressure remains elevated. He denies any symptoms and reports adherence to a healthy diet and regular exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary hypertension
- Suspected Marfan syndrome (genetic evaluation pending)

He was diagnosed with primary hypertension 6 months ago.

PAST SURGICAL HISTORY:
- Emergent ascending aortic replacement and hemiarch repair (age 20)

MEDICATIONS:
He was started on lisinopril.

Home Medications:
- Metoprolol tartrate 25 mg by mouth twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for hypertension in both parents.

SOCIAL HISTORY:
He does not smoke or use illicit drugs but admits to occasional alcohol consumption on weekends.

VITAL SIGNS:
On examination, his blood pressure is 152/96 mmHg, heart rate is 78 bpm, and BMI is 24 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is otherwise unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 140 mEq/L
- Serum potassium: 3.5 mEq/L
- Serum creatinine: 0.9 mg/dL
- Fasting blood glucose: 88 mg/dL
- Total cholesterol: 180 mg/dL
- LDL cholesterol: 110 mg/dL
- HDL cholesterol: 45 mg/dL

ASSESSMENT AND PLAN:
Given the patient's persistent hypertension,

## Encounter 1: source 2021-01-15, practice 2026-05-28, ed, Emergency Medicine

Chief complaint: comes to the emergency department due to sudden onset of severe chest pain radiating to his back

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
He comes to the emergency department due to sudden onset of severe chest pain radiating to his back

PAST MEDICAL HISTORY:
Active Problem List:
- Medication noncompliance (diagnosed 2020-01-15)
- Primary hypertension
- Suspected Marfan syndrome (genetic evaluation pending)

PAST SURGICAL HISTORY:
- Emergent ascending aortic replacement and hemiarch repair (age 20)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg by mouth daily
- Metoprolol tartrate 25 mg by mouth twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Paternal uncle: Aortic dissection (age 42)
- Father: Hypertension
- Mother: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional weekend consumption (2-3 drinks per occasion). Occupation: University student.

VITAL SIGNS:
Blood pressure is 150/95 mmHg in the right arm and 130/85 mmHg in the left arm.

PHYSICAL EXAMINATION:
He is tall and thin with long limbs.
Physical examination reveals a high-arched palate and hyperextensible joints. Cardiac auscultation reveals an early diastolic decrescendo murmur best heard along the left sternal border.

IMAGING:
Imaging shows an intimal tear in the proximal descending aorta.
